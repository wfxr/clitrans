mod cli;

use cli::{Clap, Opts};
use clitrans::{bing::Translator, Parser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let query = opts.query;
    let url = format!("https://cn.bing.com/dict/search?q={}&mkt={}", query, "zh-cn");
    let resp = reqwest::get(&url).await?.text().await?;
    let trans = Translator::parse(&resp);
    trans.print();
    Ok(())
}
