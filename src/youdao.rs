use crate::util::html::*;
use crate::*;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

pub struct Translator;

#[async_trait]
impl Translate for Translator {
    async fn translate(&self, query: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>> {
        let url = format!("http://dict.youdao.com/w/{}", query);
        let client = reqwest::Client::builder().build().unwrap();
        let resp = client
            .get(&url)
            .header("Accept-Encoding", "gzip")
            .header("Accept-Language", "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7")
            .send()
            .await?
            .text()
            .await?;
        Ok(parse(&resp))
    }
}

fn parse(body: &str) -> Option<Translation> {
    let root = Html::parse_document(&body);
    let content = get_element(&root, "#results-contents")?;
    let query = get_text(content, "#phrsListTab > h2 > .keyword")
        .into_iter()
        .next()
        .unwrap_or_else(|| {
            get_text(content, "#fanyiToggle > div > p:nth-child(1)")
                .into_iter()
                .next()
                .expect("query not found")
        });
    let prons = parse_pronounciations(content);
    let exps = parse_explanation(content);
    Some(Translation { query, prons, exps })
}

fn parse_pronounciations(detail: ElementRef) -> Vec<Pronunciation> {
    let mut prons = vec![];
    if let Some(s) = get_text(detail, "#phrsListTab > h2 > div.baav").get(0) {
        let re = Regex::new(
            r"(?x)
                (\s*英\s*\[(?P<uk>.*?)]\s*)?
                (\s*美\s*\[(?P<us>.*?)]\s*)?
            ",
        )
        .unwrap();

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
    }
    if let Some(s) = get_text(detail, "#phrsListTab > h2 > span.phonetic").get(0) {
        prons.push(Pronunciation::pinyin(
            s.as_str().trim_matches(&['[', ']'][..]).to_owned(),
        ));
    }
    prons
}

fn parse_explanation(detail: ElementRef) -> Vec<Explanation> {
    let mut exps = parse_explanation_en(detail);
    exps.extend(parse_explanation_cn(detail));
    exps.extend(parse_explanation_machine(detail));
    exps
}

fn parse_explanation_en(detail: ElementRef) -> Vec<Explanation> {
    let selector = Selector::parse("#phrsListTab > div.trans-container > ul > li").unwrap();
    let re = Regex::new(r#"(?P<pos>\w+\.)(?P<exp>.*)"#).unwrap();
    let mut exps = vec![];
    for li in detail.select(&selector) {
        let text: String = li.text().collect();
        if let Some(caps) = re.captures(&text) {
            if let (Some(pos), Some(exp)) = (caps.name("pos"), caps.name("exp")) {
                exps.push(Explanation {
                    pos:    pos.as_str().trim().to_owned(),
                    values: exp
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

fn parse_explanation_cn(detail: ElementRef) -> Vec<Explanation> {
    let mut exps = vec![];
    let selector = Selector::parse("#phrsListTab > div.trans-container > ul > p").unwrap();
    for record in detail.select(&selector) {
        let values = get_text(record, "span .search-js");
        if values.is_empty() {
            continue;
        }
        let pos = get_text(record, "span:nth-child(1):not(.contentTitle)")
            .into_iter()
            .next()
            .unwrap_or_else(|| "Phrase".to_owned());
        exps.push(Explanation {
            pos,
            values: get_text(record, "span .search-js"),
        });
    }
    exps
}

fn parse_explanation_machine(detail: ElementRef) -> Option<Explanation> {
    let value = get_text(detail, "#fanyiToggle > div > p:nth-child(2)")
        .into_iter()
        .next()?;
    Some(Explanation {
        pos:    "Machine.".to_owned(),
        values: vec![value],
    })
}
