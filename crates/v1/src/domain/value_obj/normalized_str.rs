//! 空文字禁止，NFKC正規化，最大長チェックを行う汎用VO

use crate::error::{AppError, AppResult};
use unicode_normalization::UnicodeNormalization;
