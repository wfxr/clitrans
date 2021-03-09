#![feature(format_args_capture)]
pub mod engine;
pub mod translation;
mod util;

use async_trait::async_trait;
use translation::Translation;

#[async_trait]
pub trait Translate {
    async fn translate(&self, text: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub phrases: usize,
}
