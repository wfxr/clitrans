use super::*;
use regex::Regex;
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

pub struct Translator;

#[async_trait]
impl Translate for Translator {
    async fn translate(&self, input: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>> {
        let url = get_url(input)?;
        let resp = reqwest::get(url.clone()).await?.text().await?;
        Ok(parse(url, &resp))
    }
}

fn parse(url: Url, body: &str) -> Option<Translation> {
    let root = Html::parse_document(&body);
    let content = get_element(
        &root,
        r#"
                body
                .contentPadding
                .b_cards
                .b_cards
                .lf_area
            "#,
    )?;
    let query = get_text(content, ".qdef .hd_area #headword")
        .into_iter()
        .next()
        .expect("query not found");
    let prons = parse_pronounciations(content);
    let exps = parse_explanation(content);
    Some(
        Translation::new(query, url.to_string())
            .pronunciations(prons)
            .explanations(exps),
    )
}

fn get_url(query: &str) -> Result<Url, Box<dyn std::error::Error>> {
    Ok(Url::parse(&format!(
        "https://cn.bing.com/dict/search?q={}&mkt={}",
        query, "zh-cn"
    ))?)
}

fn parse_pronounciations(detail: ElementRef) -> Vec<Pronunciation> {
    let mut prons = vec![];
    let selector = Selector::parse(".hd_p1_1").unwrap();
    if let Some(node) = detail.select(&selector).next() {
        if node.children().count() == 1 {
            let pron: String = node.text().collect();
            let re_py = Regex::new(r"\[(.*?)]").unwrap();
            if let Some(caps) = re_py.captures(&pron) {
                prons.push(Pronunciation::pinyin(caps[1].to_owned()));
            }
        } else {
            let re_us = Regex::new(r"美\s*\[(.*?)]").unwrap();
            let re_uk = Regex::new(r"英\s*\[(.*?)]").unwrap();
            let re_audio = Regex::new("https?://.*?.mp3").unwrap();
            let selector = Selector::parse(".hd_p1_1 div").unwrap();
            let mut it = detail.select(&selector);
            while let Some(div) = it.next() {
                let pron: String = div.text().collect();
                let audio = it.next().and_then(|div| {
                    div.children().next().and_then(|a| {
                        a.value().as_element().unwrap().attr("onclick").and_then(|s| {
                            re_audio
                                .captures(s)
                                .and_then(|caps| caps.get(0).map(|url| url.as_str().to_owned()))
                        })
                    })
                });
                if let Some(caps) = re_us.captures(&pron) {
                    prons.push(Pronunciation::us(caps[1].to_owned()).audio(audio));
                } else if let Some(caps) = re_uk.captures(&pron) {
                    prons.push(Pronunciation::uk(caps[1].to_owned()).audio(audio));
                }
            }
        }
    }
    prons
}

fn parse_explanation(detail: ElementRef) -> Vec<Explanation> {
    let s_li = Selector::parse(".qdef ul li").unwrap();
    let s_pos = Selector::parse(".pos").unwrap();
    let s_def = Selector::parse(".def").unwrap();
    let mut exps = vec![];
    for li in detail.select(&s_li) {
        let pos: String = li.select(&s_pos).next().expect("pos not found").text().collect();
        let def: String = li.select(&s_def).next().expect("def not found").text().collect();
        let tag = match pos.trim() {
            "网络" => ExpTag::Web,
            s => ExpTag::Pos(s.to_owned()),
        };
        let items = def.split(&['；', ';'][..]).map(|v| v.trim().to_owned()).collect();
        exps.push(Explanation { tag, items });
    }
    exps
}
