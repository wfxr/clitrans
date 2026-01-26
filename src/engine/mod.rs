pub mod bing;
pub mod youdao;

use super::{
    Result,
    Translate,
    translation::{ExpTag, Explanation, Pronunciation, Translation},
    util::html::*,
};

#[macro_export]
macro_rules! regex {
    ($init:expr) => {{
        use regex::Regex;
        use std::sync::LazyLock;
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new($init).unwrap());
        LazyLock::force(&RE)
    }};
}
