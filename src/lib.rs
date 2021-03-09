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
    query:   String,
    prons:   Vec<Pronunciation>,
    exps:    Vec<Explanation>,
    phrases: Vec<(String, Vec<String>)>,
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

        let tag_width = exps
            .iter()
            .map(|&(_, tag, _)| UnicodeWidthStr::width_cjk(tag))
            .max()
            .unwrap_or(0);
        if !exps.is_empty() {
            let buf = exps
                .into_iter()
                .map(|(color, tag, itmes)| {
                    itmes
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            let title = if i == 0 { tag } else { "" };
                            format!(
                                "{:>width$}  {} {}",
                                title.color(color).italic(),
                                "*".color(color),
                                item.color(color),
                                width = tag_width
                            )
                        })
                        .join("\n")
                })
                .join("\n\n");
            println!("\n{}", buf);
        }

        if !self.phrases.is_empty() {
            let buf = self
                .phrases
                .iter()
                .map(|(phrase, exps)| {
                    format!("{:>width$}  {} {}\n", "", "*", phrase, width = tag_width)
                        + &exps
                            .iter()
                            .map(|exp| format!("{:>width$}    - {}", "", exp.purple(), width = tag_width))
                            .join("\n")
                })
                .join("\n\n");
            println!("\n{}\n{}", "Web Phrases:".cyan(), buf);
        }
    }
}
