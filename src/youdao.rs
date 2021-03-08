use crate::util::html::*;
use crate::*;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

pub struct Translator;

#[async_trait]
impl Translate for Translator {
    async fn translate(&self, query: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>> {
        let url = format!("http://dict.youdao.com/w/{}", query);
        let resp = reqwest::get(&url).await?.text().await?;
        Ok(parse(&resp))
    }
}

fn parse(body: &str) -> Option<Translation> {
    let root = Html::parse_document(&body);
    let content = get_element(&root, "#results-contents")?;
    let query = get_text(content, "#phrsListTab > h2 > .keyword").expect("query not found");
    let prons = parse_pronounciations(content);
    let exps = parse_explanation(content);
    Some(Translation {
        query,
        prons,
        exps,
        mexp: None,
    })
}

// TODO: Fix pinyin <21-03-08 19:57, Wenxuan Zhang> //
fn parse_pronounciations(detail: ElementRef) -> Vec<Pronunciation> {
    let s = get_text(detail, "#phrsListTab > h2 > div.baav").expect("prons not found");
    let re = Regex::new(
        r"(?x)
        (\s*英\s*\[(?P<uk>.*?)]\s*)?
        (\s*美\s*\[(?P<us>.*?)]\s*)?
        ",
    )
    .unwrap();
    let mut prons = vec![];

    if let Some(caps) = re.captures(&s) {
        if let Some(py) = caps.name("py") {
            prons.push(Pronunciation::pinyin(py.as_str().to_owned()));
        }
        if let Some(us) = caps.name("us") {
            prons.push(Pronunciation::us(us.as_str().to_owned()));
        }
        if let Some(uk) = caps.name("uk") {
            prons.push(Pronunciation::uk(uk.as_str().to_owned()));
        }
    }
    prons
}

fn parse_explanation(detail: ElementRef) -> Vec<Explanation> {
    let s_li = Selector::parse("#phrsListTab > div.trans-container > ul > li").unwrap();
    let re = Regex::new(r#"(?P<prop>\w+\.)(?P<exp>.*)"#).unwrap();
    let mut exps = vec![];
    for li in detail.select(&s_li) {
        let text: String = li.text().collect();
        if let Some(caps) = re.captures(&text) {
            if let (Some(prop), Some(exp)) = (caps.name("prop"), caps.name("exp")) {
                exps.push(Explanation {
                    prop:  prop.as_str().trim().to_owned(),
                    value: exp
                        .as_str()
                        .split(&['；', ';'][..])
                        .map(|v| v.trim().to_owned())
                        .collect(),
                });
            }
        }
    }
    exps
}
