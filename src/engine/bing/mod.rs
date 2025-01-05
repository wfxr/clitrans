#[cfg(test)]
mod test;

use std::sync::LazyLock;

use super::*;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

#[derive(Clone)]
pub struct Translator;

impl Translate for Translator {
    fn translate(&self, input: &str) -> Result<Option<Translation>> {
        let uri = format_url!(
            "https://cn.bing.com/dict/search?q={}&mkt={}",
            input,
            "zh-cn"
        )?
        .to_uri()?;
        let resp = Request::get(&uri)
            .header("Accept-Encoding", "gzip")
            .header("Accept-Language", "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7")
            .body(())?
            .send()?
            .text()?;
        Ok(parse(&uri, &resp))
    }
}

fn parse(url: &Uri, body: &str) -> Option<Translation> {
    let root = Html::parse_document(body);
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

fn parse_pronounciations(detail: ElementRef) -> Vec<Pronunciation> {
    static RE_PY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(.*?)]").unwrap());
    static RE_US: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"US\s*\[(.*?)]").unwrap());
    static RE_UK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"UK\s*\[(.*?)]").unwrap());
    static RE_MP3: LazyLock<Regex> = LazyLock::new(|| Regex::new("https?://.*?.mp3").unwrap());
    let mut prons = vec![];
    let selector = Selector::parse(".hd_p1_1").unwrap();
    if let Some(node) = detail.select(&selector).next() {
        if node.children().count() == 1 {
            let pron: String = node.text().collect();
            if let Some(caps) = RE_PY.captures(&pron) {
                prons.push(Pronunciation::pinyin(caps[1].to_owned()));
            }
        } else {
            let selector = Selector::parse(".hd_p1_1 div").unwrap();
            let mut it = detail.select(&selector);
            while let Some(div) = it.next() {
                let pron: String = div.text().collect();
                let audio = it.next().and_then(|div| {
                    div.children().next().and_then(|a| {
                        a.value()
                            .as_element()
                            .unwrap()
                            .attr("onclick")
                            .and_then(|s| {
                                RE_MP3
                                    .captures(s)
                                    .and_then(|caps| caps.get(0).map(|url| url.as_str().to_owned()))
                            })
                    })
                });
                if let Some(caps) = RE_US.captures(&pron) {
                    prons.push(Pronunciation::us(caps[1].to_owned()).audio(audio));
                } else if let Some(caps) = RE_UK.captures(&pron) {
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
        let pos: String = li
            .select(&s_pos)
            .next()
            .expect("pos not found")
            .text()
            .collect();
        let def: String = li
            .select(&s_def)
            .next()
            .expect("def not found")
            .text()
            .collect();
        let tag = match pos.trim() {
            "网络" => ExpTag::Web,
            s => ExpTag::Pos(s.to_owned()),
        };
        let items = def
            .split(&['；', ';'][..])
            .map(|v| v.trim().to_owned())
            .collect();
        exps.push(Explanation { tag, items });
    }
    exps
}
