pub mod bing;
pub mod youdao;

use super::{
    translation::{ExpTag, Explanation, Pronunciation, Translation},
    util::html::*,
    Result,
    Translate,
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
