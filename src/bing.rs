use crate::*;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

pub struct Translator;

impl Parser for Translator {
    fn parse(body: &str) -> Translation {
        let root = Html::parse_document(&body);
        let detail = get_detail(&root).expect("detail not found");
        let query = get_query(&detail).expect("query not found");
        let prons = get_prons(detail);
        let exps = get_explanation(detail);
        Translation { query, prons, exps }
    }
}

fn get_detail(doc: &Html) -> Option<ElementRef> {
    let selector = Selector::parse(
        r#"
            body
            .contentPadding
            .b_cards
            .b_cards
            .lf_area
            "#,
    )
    .unwrap();
    doc.select(&selector).next()
}

fn get_query(detail: &ElementRef) -> Option<String> {
    let selector = Selector::parse(
        r#"
            .qdef
            .hd_area
            #headword
        "#,
    )
    .unwrap();
    Some(detail.select(&selector).next()?.text().collect())
}

fn get_prons(detail: ElementRef) -> Vec<Pronunciation> {
    let selector = Selector::parse(".hd_p1_1").unwrap();
    let s: String = detail
        .select(&selector)
        .next()
        .expect("prons not found")
        .text()
        .collect();
    let re = Regex::new(
        r"(?x)
        (\s*\[(?P<py>.*?)]\s*)?
        (美\s*\[(?P<us>.*?)]\s*)?
        (英\s*\[(?P<uk>.*?)]\s*)?
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

fn get_explanation(detail: ElementRef) -> Vec<Explanation> {
    let s_li = Selector::parse(".qdef ul li").unwrap();
    let s_pos = Selector::parse(".pos").unwrap();
    let s_def = Selector::parse(".def").unwrap();
    let mut exps = vec![];
    for li in detail.select(&s_li) {
        let mut pos: String = li.select(&s_pos).next().expect("pos not found").text().collect();
        let def: String = li.select(&s_def).next().expect("def not found").text().collect();
        if pos == "网络" {
            pos = "Web.".to_owned()
        }
        exps.push(Explanation {
            prop:  pos,
            value: def.split(&['；', ';'][..]).map(|v| v.trim().to_owned()).collect(),
        });
    }
    exps
}
