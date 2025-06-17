//! 空文字禁止，NFKC正規化，最大長チェックを行う汎用VO

use crate::error::AppError;
use crate::error::AppResult;
use std::borrow::Cow;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedString {
    value: String,
}

impl NormalizedString {
    /// # Constructor
    ///
    /// ## @param
    /// - `input`: 入力文字列（&strまたはString）
    /// - `required`: true := 空文字列を許容しない。
    /// - `target`: エラーメッセージ用のパラメータ名
    /// - `min_len`: 最小文字数（Noneの場合は制限なし）
    /// - `max_len`: 最大文字数（Noneの場合は制限なし）
    ///
    /// ## processing
    /// - NFKC正規化 & trim
    /// - `required`がtrueの場合は，エラーを返す。
    /// - 文字数がmin_len未満又はmax_lenを超える場合はエラーを返す。
    ///
    /// ## @result
    /// - 正常時：正規化済みの入力が空でなければSome(NormalizedString)を返す。
    /// - `required`がfalseの場合かつ、正規化済みのinputが空文字列の場合はNoneを返す。
    /// - 異常時：AppErrorを返す。
    pub fn new<S: AsRef<str>>(
        // S = StringにInto可能な値(&str, String)
        input: S,
        required: bool,
        target: &str,
        min_len: Option<usize>,
        max_len: Option<usize>,
    ) -> AppResult<Option<Self>> {
        // Cow<str>を使って、&strならcloneせず、Stringなら所有権を奪う
        let input_cow: Cow<str> = Cow::Borrowed(input.as_ref());

        // 文字列の正規化
        // NFKC正規化・trim処理
        // trim()は&strを返すため、to_string()でStringに戻す。
        let normalized = input_cow.nfkc().collect::<String>().trim().to_string();

        // 値が存在するかを確認する。
        if normalized.is_empty() {
            // 値が存在しない場合，そのパラメータが必須パラメータである場合はエラーを返す。
            return if required {
                Err(AppError::UnprocessableContent(Some(format!(
                    "{target}は必須のパラメータです。"
                ))))
            } else {
                Ok(None)
            };
        }
        // グラフェム単位で文字列長をカウントする。
        let graphemes = normalized.graphemes(true);
        let len = graphemes.count();

        // 最小文字列長が定義されている場合
        if let Some(min) = min_len {
            if len < min {
                return Err(AppError::UnprocessableContent(Some(format!(
                    "{target}は{min}文字以上で入力してください。"
                ))));
            }
        }

        // 最大文字列長が定義されている場合
        if let Some(max) = max_len {
            if len > max {
                return Err(AppError::UnprocessableContent(Some(format!(
                    "{target}は{max}文字以内で入力してください。"
                ))));
            }
        }
        //
        Ok(Some(Self { value: normalized }))
    }
    /// 正規化済みの入力文字列スライスを返す。
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::value_obj::normalized_string::NormalizedString;

    #[test]
    fn normalizes_nfkc_differently_composed_characters() {
        let input = "デデ";
        let result = NormalizedString::new(input, true, "name", None, None).unwrap();
        assert_ne!(result.unwrap().as_str(), input);
    }

    #[test]
    fn normalizes_nfkc_and_trims_spaces_and_wide_chars() {
        let input = "　　　　　　１２３ａｂｃｱｲｳｴｵ①㈱㌖       ";
        let result = NormalizedString::new(input, true, "name", None, None).unwrap();
        assert_eq!(
            result.unwrap().as_str(),
            "123abcアイウエオ1(株)キロメートル"
        );
    }

    #[test]
    fn normalizes_nfkc_3() {
        let input = "（）．，「」。、().,｢｣｡､";
        let result = NormalizedString::new(input, true, "name", None, None).unwrap();
        assert_eq!(result.unwrap().as_str(), "().,「」。、().,「」。、");
    }
    #[test]
    fn returns_none_when_optional_and_empty_after_normalization() {
        let input = "  　　";
        let result = NormalizedString::new(input, false, "name", None, None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn returns_error_when_required_and_empty_after_normalization() {
        let input = "  　　";
        let err = NormalizedString::new(input, true, "name", None, None).unwrap_err();
        assert!(format!("{err:?}").contains("必須のパラメータ"));
    }

    #[test]
    fn returns_error_when_below_min_length() {
        let input = "abcd";
        let err = NormalizedString::new(input, true, "name", Some(5), None).unwrap_err();
        assert!(format!("{err:?}").contains("5文字以上"));
    }

    #[test]
    fn returns_error_when_above_max_length() {
        let input = "abcdef";
        let err = NormalizedString::new(input, true, "name", None, Some(5)).unwrap_err();
        assert!(format!("{err:?}").contains("5文字以内"));
    }

    #[test]
    fn accepts_exact_min_and_max_length() {
        let input = "abcde";
        let result = NormalizedString::new(input, true, "name", Some(5), Some(5)).unwrap();
        assert_eq!(result.unwrap().as_str(), "abcde");
    }

    #[test]
    fn counts_grapheme_clusters_correctly() {
        // "👨‍👩‍👧‍👦" is a single grapheme cluster but multiple code points
        let input = "👨‍👩‍👧‍👦";
        let result = NormalizedString::new(input, true, "emoji", Some(1), Some(1)).unwrap();
        assert_eq!(result.unwrap().as_str(), input);
    }

    #[test]
    fn trims_and_normalizes_mixed_input() {
        let input = "　ＡＢＣ　abc　";
        let result = NormalizedString::new(input, true, "mixed", None, None).unwrap();
        assert_eq!(result.unwrap().as_str(), "ABCabc");
    }

    #[test]
    fn works_with_owned_string() {
        let input = String::from("  １２３  ");
        let result = NormalizedString::new(input, true, "number", None, None).unwrap();
        assert_eq!(result.unwrap().as_str(), "123");
    }
}
