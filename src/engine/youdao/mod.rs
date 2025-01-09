#[cfg(test)]
mod test;

use std::sync::LazyLock;

use super::*;
use itertools::Itertools;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use url::Url;

#[derive(Clone)]
pub struct Translator;

impl Translate for Translator {
    fn translate(&self, query: &str) -> Result<Option<Translation>> {
        let url: Url = format!("http://dict.youdao.com/w/{query}").parse()?;
        let resp = ureq::request_url("GET", &url)
            .set("Accept-Encoding", "gzip")
            .set("Accept-Language", "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7")
            .call()?
            .into_string()?;
        Ok(parse(url, &resp))
    }
}

fn parse(url: Url, body: &str) -> Option<Translation> {
    let root = Html::parse_document(body);
    let content = get_element(&root, "#results-contents")?;
    let query = get_text(content, "#phrsListTab > h2 > .keyword")
        .into_iter()
        .next()
        .or_else(|| {
            get_text(content, "#fanyiToggle > div > p:nth-child(1)")
                .into_iter()
                .next()
        })?;
    let prons = parse_pronounciations(content);
    let exps = parse_explanation(content);
    let phrases = parse_phrases(content);
    Some(
        Translation::new(query, url.to_string())
            .pronunciations(prons)
            .explanations(exps)
            .phrases(phrases),
    )
}

fn parse_phrases(content: ElementRef) -> Vec<(String, Vec<String>)> {
    let mut rs = vec![];
    for item in content.select(&Selector::parse("#webPhrase > p.wordGroup").unwrap()) {
        if let Some(title) = get_text(item, ".contentTitle").into_iter().next() {
            let text: String = item.text().collect();
            let items = text
                .replacen(&title, "", 1)
                .split(&[';', '；'][..])
                .map(|s| s.split_whitespace().join(" "))
                .unique()
                .collect();
            rs.push((title, items));
        }
    }
    rs
}

fn parse_pronounciations(detail: ElementRef) -> Vec<Pronunciation> {
    static RE_US: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*美\s*\[(.*?)]").unwrap());
    static RE_UK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*英\s*\[(.*?)]").unwrap());
    let mut prons = vec![];
    let pron_selector = Selector::parse("#phrsListTab > h2 > div.baav > .pronounce").unwrap();
    let audio_selector = Selector::parse("a.dictvoice").unwrap();
    for pron in detail.select(&pron_selector) {
        let text: String = pron.text().collect();
        let audio = pron
            .select(&audio_selector)
            .next()
            .and_then(|a| a.value().attr("data-rel"))
            .map(|data_rel| format!("https://dict.youdao.com/dictvoice?audio={data_rel}"));
        if let Some(caps) = RE_US.captures(&text) {
            prons.push(Pronunciation::us(caps[1].to_owned()).audio(audio));
        } else if let Some(caps) = RE_UK.captures(&text) {
            prons.push(Pronunciation::uk(caps[1].to_owned()).audio(audio));
        }
    }
    if let Some(s) = get_text(detail, "#phrsListTab > h2 > span.phonetic").first() {
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
    exps.extend(parse_explanation_web(detail));
    exps
}

fn parse_explanation_en(detail: ElementRef) -> Vec<Explanation> {
    static RE_EXP: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(?P<pos>\w+\.)?(?P<exp>.*)"#).unwrap());
    let selector = Selector::parse("#phrsListTab > div.trans-container > ul > li").unwrap();
    let mut exps = vec![];
    for li in detail.select(&selector) {
        let text: String = li.text().collect();
        if let Some(caps) = RE_EXP.captures(&text) {
            if let Some(exp) = caps.name("exp") {
                let tag = caps
                    .name("pos")
                    .map(|pos| ExpTag::Pos(pos.as_str().trim().to_owned()))
                    .unwrap_or(ExpTag::Phrase);
                let items = exp
                    .as_str()
                    .split(&['；', ';'][..])
                    .map(|v| v.trim().to_owned())
                    .collect();
                exps.push(Explanation { tag, items });
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
        let tag = get_text(record, "span:nth-child(1):not(.contentTitle)")
            .into_iter()
            .next()
            .map(ExpTag::Pos)
            .unwrap_or(ExpTag::Phrase);
        let items = get_text(record, "span .search-js");
        exps.push(Explanation { tag, items });
    }
    exps
}

fn parse_explanation_machine(detail: ElementRef) -> Option<Explanation> {
    let value = get_text(detail, "#fanyiToggle > div > p:nth-child(2)")
        .into_iter()
        .next()?;
    Some(Explanation { tag: ExpTag::Machine, items: vec![value] })
}

fn parse_explanation_web(detail: ElementRef) -> Vec<Explanation> {
    let texts = get_text(detail, "#tWebTrans > div.wt-container > .title");
    let items = texts
        .iter()
        .map(|s| s.split_whitespace().join(" "))
        .collect();
    vec![Explanation { tag: ExpTag::Web, items }]
}
