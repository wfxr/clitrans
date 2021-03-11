pub mod bing;
pub mod youdao;

use super::translation::{ExpTag, Explanation, Pronunciation, Translation};
use super::util::html::*;
use super::Translate;

use isahc::{
    http::{self, Request, Uri},
    HttpClientBuilder, ReadResponseExt,
};
use url::Url;

use crate::format_url;

#[macro_export]
macro_rules! format_url {
    ($($tt:tt)*) => {{
        Url::parse(&format!($($tt)*))
    }};
}

trait ToUri {
    fn to_uri(&self) -> Result<Uri, http::uri::InvalidUri>;
}

impl ToUri for Url {
    fn to_uri(&self) -> Result<http::Uri, http::uri::InvalidUri> {
        self.to_string().parse()
    }
}
