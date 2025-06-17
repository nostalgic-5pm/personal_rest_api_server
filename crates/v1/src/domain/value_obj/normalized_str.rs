//! 空文字禁止，NFKC正規化，最大長チェックを行う汎用VO

use crate::error::{AppError, AppResult};
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
    /// - 正常時：Some(NormalizedString)を返す。
    /// - - `required`がfalse場合かつ、正規化済みのinputが空文字列の場合はNoneを返す。
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
        let input_cow: Cow<str> = match input.as_ref() {
            s => Cow::Borrowed(s),
        };

        // 文字列の正規化
        // NFKC正規化し、前後の空白を除去
        // （NFKC正規化後にtrim()を適用することで、正規化によって生じる前後の空白も除去できる。）
        // trim()は&strを返すため、to_owned()でStringに戻す。
        let normalized = input_cow.nfkc().collect::<String>().trim().to_owned();

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
