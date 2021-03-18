pub mod engine;
pub mod translation;
mod util;

#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
use anyhow::Result;

pub use translation::Translation;

pub trait Translate: Send + Clone {
    fn translate(&self, text: &str) -> Result<Option<Translation>>;
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub explanations: usize,
    pub phonetics:    usize,
    pub phrases:      usize,
}
