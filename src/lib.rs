pub mod bing;
mod util;
pub mod youdao;

pub use async_trait::async_trait;
use colored::{Color, Colorize};
use itertools::Itertools;
use unicode_width::UnicodeWidthStr;

#[async_trait]
pub trait Translate {
    async fn translate(&self, text: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct Translation {
    query: String,
    prons: Vec<Pronunciation>,
    exps:  Vec<Explanation>,
}

#[derive(Debug)]
struct Pronunciation {
    region: &'static str,
    value:  String,
}

impl Pronunciation {
    fn pinyin(value: String) -> Self {
        Self { region: "CN", value }
    }
    fn us(value: String) -> Self {
        Self { region: "US", value }
    }
    fn uk(value: String) -> Self {
        Self { region: "UK", value }
    }
}

#[derive(Debug)]
pub struct Explanation {
    tag:   ExpTag,
    items: Vec<String>,
}

#[derive(Debug)]
pub enum ExpTag {
    Web,
    Machine,
    Phrase,
    Pos(String),
}

impl Translation {
    pub fn print(&self) {
        println!("{}", self.query);
        if !self.prons.is_empty() {
            println!(
                "{}",
                self.prons
                    .iter()
                    .map(|pron| &pron.value)
                    .unique()
                    .map(|s| format!("/{}/", s.yellow()))
                    .join(", ")
            );
        }

        let exps: Vec<_> = self
            .exps
            .iter()
            .map(|exp| {
                #[rustfmt::skip]
                let (color, tag) = match &exp.tag {
                    ExpTag::Web     => (Color::Magenta,     "Web."),
                    ExpTag::Machine => (Color::Cyan,        "Machine."),
                    ExpTag::Phrase  => (Color::Green,       "Phrase."),
                    ExpTag::Pos(s)  => (Color::BrightGreen, s.as_str()),
                };
                (color, tag, &exp.items)
            })
            .collect();

        if !exps.is_empty() {
            println!();
            let tag_width = exps
                .iter()
                .map(|&(_, tag, _)| UnicodeWidthStr::width_cjk(tag))
                .max()
                .unwrap_or(0);
            for (color, tag, itmes) in exps {
                for (i, item) in itmes.iter().enumerate() {
                    let title = if i == 0 { tag } else { "" };
                    println!(
                        "{:>width$}  {} {}",
                        title.color(color).italic(),
                        "*".color(color),
                        item.color(color),
                        width = tag_width
                    )
                }
                println!()
            }
        }
    }
}
