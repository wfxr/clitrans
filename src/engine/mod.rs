pub mod bing;
pub mod youdao;

use super::translation::{ExpTag, Explanation, Pronunciation, Translation};
use super::util::html::*;
use super::{async_trait, Translate};
