#![feature(async_closure)]
mod cli;

use std::process;

use cli::*;
use clitrans::{engine::*, Layout, Translate};

#[tokio::main]
async fn main() {
    if let Err(e) = async || -> Result<(), Box<dyn std::error::Error>> {
        let opts: Opts = Opts::parse();
        let query = opts.query;
        let engine: Box<dyn Translate> = match opts.engine {
            Engine::bing => Box::new(bing::Translator),
            Engine::youdao => Box::new(youdao::Translator),
        };

        let layout = Layout { phrases: opts.phrases };
        match engine.translate(&query).await? {
            Some(trans) => {
                trans.print(&layout);
                if let Some(tag) = opts.audio {
                    trans.play_audio(&tag).await?;
                }
            }
            None => return Err("translation not found".into()),
        }
        Ok(())
    }()
    .await
    {
        eprintln!();
        eprintln!("Messages:");
        eprintln!("  * {}", e);
        process::exit(1);
    }
}
