use cli_trans::{bing::Translator, Parser};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = env::args().nth(1).expect("missing query");
    let url = format!("https://cn.bing.com/dict/search?q={}&mkt={}", query, "zh-cn");
    let resp = reqwest::get(&url).await?.text().await?;
    let trans = Translator::parse(&resp);
    trans.print();
    Ok(())
}
