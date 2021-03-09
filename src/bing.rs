use crate::util::html::*;
use crate::*;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

pub struct Translator;

#[async_trait]
impl Translate for Translator {
    async fn translate(&self, query: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>> {
        let url = format!("https://cn.bing.com/dict/search?q={}&mkt={}", query, "zh-cn");
        let resp = reqwest::get(&url).await?.text().await?;
        Ok(parse(&resp))
    }
}

fn parse(body: &str) -> Option<Translation> {
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
    Some(Translation {
        query,
        prons,
        exps,
        phrases: vec![],
    })
}

fn parse_pronounciations(detail: ElementRef) -> Vec<Pronunciation> {
    let s = get_text(detail, ".hd_p1_1")
        .into_iter()
        .next()
        .expect("prons not found");
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
