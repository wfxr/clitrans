use colored::Colorize;
use regex::Regex;
use scraper::{Html, Selector};
use std::env;

fn get_content(doc: &Html) -> Option<&str> {
    let selector = Selector::parse(r#"meta[name="description"]"#).unwrap();
    doc.select(&selector).next()?.value().attr("content")
}

#[derive(Debug)]
struct Translation<'a> {
    prons:   Vec<Pronunciation<'a>>,
    exps:    Vec<Explanation<'a>>,
    web_exp: Option<&'a str>,
}

#[derive(Debug)]
struct Pronunciation<'a> {
    region: &'a str,
    value:  &'a str,
}

impl<'a> Pronunciation<'a> {
    fn pinyin(value: &'a str) -> Self {
        Self { region: "CN", value }
    }
    fn us(value: &'a str) -> Self {
        Self { region: "US", value }
    }
    fn uk(value: &'a str) -> Self {
        Self { region: "UK", value }
    }
}

#[derive(Debug)]
struct Explanation<'a> {
    prop:  &'a str,
    value: &'a str,
}

impl<'a> Translation<'_> {
    fn print(&self) {
        if !self.prons.is_empty() {
            for pron in &self.prons {
                print!("{}: [{}] ", pron.region, pron.value.yellow().bold())
            }
            println!();
        }

        println!();
        println!("Explanation:");
        for exp in &self.exps {
            println!("{:>6}  {}", exp.prop.italic().green(), exp.value.green())
        }
        if let Some(web_exp) = self.web_exp {
            println!("{:>6}  {}", "web".italic().purple(), web_exp.purple())
        }
    }
}

fn parse_content(s: &'_ str) -> Translation<'_> {
    let re = Regex::new(
        r"(?x)
        必应词典为您提供.*?的释义，
        (拼音(?P<pinyin>\[.*?])，)?
        (美\[(?P<us>.*?)]，)?
        (英\[(?P<uk>.*?)]，)?
        (?P<exp>.*)?
        网络释义：(?P<web_exp>.*)+
        ",
    )
    .unwrap();
    let caps = re.captures(s).unwrap();

    let mut prons = vec![];
    if let Some(pinyin) = caps.name("pinyin") {
        prons.push(Pronunciation::pinyin(pinyin.as_str()))
    }
    if let Some(us) = caps.name("us") {
        prons.push(Pronunciation::us(us.as_str()))
    }
    if let Some(uk) = caps.name("uk") {
        prons.push(Pronunciation::uk(uk.as_str()))
    }

    let mut exps = vec![];
    caps.name("exp").map(|exp| {
        let mut parts = exp.as_str().trim().split_ascii_whitespace();
        while let (Some(prop), Some(exp)) = (parts.next(), parts.next()) {
            exps.push(Explanation { prop, value: exp })
        }
    });
    let web_exp = caps.name("web_exp").map(|exp| exp.as_str().trim());

    Translation { prons, web_exp, exps }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = env::args().nth(1).expect("missing query");
    let url = format!("https://cn.bing.com/dict/search?q={}", query);
    let resp = reqwest::get(&url).await?.text().await?;
    let document = Html::parse_document(&resp);

    let content = get_content(&document).expect("no content found");

    let translation = parse_content(content);
    translation.print();
    Ok(())
}
