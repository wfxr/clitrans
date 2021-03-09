mod cli;

use std::process;

use cli::{Clap, Engine, Opts};
use clitrans::{engine::*, Layout, Translate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let query = opts.query;
    let engine: Box<dyn Translate> = match opts.engine {
        Engine::bing => Box::new(bing::Translator),
        Engine::youdao => Box::new(youdao::Translator),
    };

    let layout = Layout { phrases: opts.phrases };
    match engine.translate(&query).await? {
        Some(trans) => trans.print(&layout),
        None => {
            eprintln!("translation not found");
            process::exit(exitcode::UNAVAILABLE);
        }
    }
    Ok(())
}
