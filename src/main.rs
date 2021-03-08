mod cli;

use std::process;

use cli::{Clap, Opts};
use clitrans::{bing::Translator, Parser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let query = opts.query;
    let url = format!("https://cn.bing.com/dict/search?q={}&mkt={}", query, "zh-cn");
    let resp = reqwest::get(&url).await?.text().await?;
    match Translator::parse(&resp) {
        Some(trans) => trans.print(),
        None => {
            eprintln!("translation not found");
            process::exit(exitcode::UNAVAILABLE);
        }
    }
    Ok(())
}
