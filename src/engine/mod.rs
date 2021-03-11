pub mod bing;
pub mod youdao;

use super::translation::{ExpTag, Explanation, Pronunciation, Translation};
use super::util::html::*;
use super::Translate;

use isahc::{
    http::{Request, Uri},
    HttpClientBuilder, ReadResponseExt,
};
