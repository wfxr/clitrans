pub mod engine;
pub mod translation;
mod util;

pub use translation::Translation;

pub trait Translate {
    fn translate(&self, text: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub explanations: usize,
    pub phonetics:    usize,
    pub phrases:      usize,
}
