#[cfg(feature = "audio")]
use crate::util::audio::play_audio;

use super::{Layout, Result};
use colored::{Color, Colorize};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Translation {
    pub query:   String,
    pub url:     String,
    pub prons:   Vec<Pronunciation>,
    pub exps:    Vec<Explanation>,
    pub phrases: Vec<(String, Vec<String>)>,
}

impl Translation {
    pub fn new(query: String, url: String) -> Self {
        Self {
            query,
            url,
            prons: Vec::new(),
            exps: Vec::new(),
            phrases: Vec::new(),
        }
    }

    pub fn pronunciations(mut self, prons: Vec<Pronunciation>) -> Self {
        self.prons = prons;
        self
    }
    pub fn explanations(mut self, exps: Vec<Explanation>) -> Self {
        self.exps = exps;
        self
    }
    pub fn phrases(mut self, phrases: Vec<(String, Vec<String>)>) -> Self {
        self.phrases = phrases;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pronunciation {
    pub tag:   String,
    pub value: String,
    pub audio: Option<String>,
}

impl Pronunciation {
    pub fn pinyin(value: String) -> Self {
        Self { tag: "CN".to_owned(), value, audio: None }
    }
    pub fn us(value: String) -> Self {
        Self { tag: "US".to_owned(), value, audio: None }
    }
    pub fn uk(value: String) -> Self {
        Self { tag: "UK".to_owned(), value, audio: None }
    }
    pub fn audio(mut self, url: Option<String>) -> Self {
        self.audio = url;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Explanation {
    pub tag:   ExpTag,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpTag {
    Web,
    Machine,
    Phrase,
    Pos(String),
}

impl Translation {
    #[cfg(feature = "audio")]
    pub fn play_audio(&self, tag: &str) -> Result<()> {
        use anyhow::bail;

        match self
            .prons
            .iter()
            .find(|p| p.tag.to_uppercase() == tag.to_uppercase())
            .and_then(|p| p.audio.as_ref())
        {
            Some(url) => play_audio(url),
            None => {
                let possibles = self
                    .prons
                    .iter()
                    .filter_map(|p| p.audio.as_ref().map(|_| &p.tag))
                    .join(", ");
                if possibles.is_empty() {
                    bail!("audio not found")
                } else {
                    bail!(
                        "audio not found for '{}'; possible values: [{}]",
                        tag,
                        possibles
                    )
                };
            }
        }
    }

    pub fn print(&self, layout: &Layout) -> Result<()> {
        self.print_query()?;
        self.print_pronunciations(layout)?;

        let exps: Vec<_> = self
            .exps
            .iter()
            .filter(|exp| !exp.items.is_empty())
            .map(|exp| {
                #[rustfmt::skip]
                let (color, tag) = match &exp.tag {
                    ExpTag::Web     => (Color::Magenta,     "Web."),
                    ExpTag::Machine => (Color::Cyan,        "Machine."),
                    ExpTag::Phrase  => (Color::Green,       "Phrase."),
                    ExpTag::Pos(s)  => (Color::BrightGreen, s.as_str()),
                };
                (color, tag, exp.items.as_slice())
            })
            .collect();
        let indent = exps.iter().map(|&(_, tag, _)| tag.len()).max().unwrap_or(0);

        self.print_explanations(layout, indent, &exps)?;
        self.print_phrases(layout, indent)?;
        self.print_link(indent)?;
        Ok(())
    }

    fn print_query(&self) -> io::Result<()> {
        writeln!(io::stdout().lock(), "{}", self.query)
    }

    fn print_pronunciations(&self, layout: &Layout) -> io::Result<()> {
        if !self.prons.is_empty() && layout.phonetics > 0 {
            let buf = self
                .prons
                .iter()
                .take(layout.phonetics)
                .map(|pron| &pron.value)
                .unique()
                .map(|s| format!("/{}/", s.yellow()))
                .join(", ");
            return writeln!(io::stdout().lock(), "{buf}");
        }
        Ok(())
    }

    fn print_explanations(
        &self,
        layout: &Layout,
        indent: usize,
        exps: &[(Color, &str, &[String])],
    ) -> io::Result<()> {
        if !exps.is_empty() && layout.explanations > 0 {
            let buf = exps
                .iter()
                .take(layout.explanations)
                .map(|(color, tag, itmes)| {
                    itmes
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            let title = if i == 0 { tag } else { "" };
                            format!(
                                "{:>w$}  {} {}",
                                title.color(*color).italic(),
                                "*".color(*color),
                                item.color(*color),
                                w = indent,
                            )
                        })
                        .join("\n")
                })
                .join("\n\n");
            return writeln!(io::stdout().lock(), "\n{buf}");
        }
        Ok(())
    }

    fn print_phrases(&self, layout: &Layout, indent: usize) -> io::Result<()> {
        if !self.phrases.is_empty() && layout.phrases > 0 {
            let buf = self
                .phrases
                .iter()
                .filter(|(_, exps)| !exps.is_empty())
                .take(layout.phrases)
                .map(|(phrase, exps)| {
                    format!("{:>w$}  {} {}\n", "", "*".cyan(), phrase.cyan(), w = indent)
                        + &exps
                            .iter()
                            .map(|exp| {
                                format!("{:>w$}    {} {}", "", "-".cyan(), exp.cyan(), w = indent)
                            })
                            .join("\n")
                })
                .join("\n\n");
            return writeln!(io::stdout().lock(), "\n{}\n{}", "Web Phrases:".cyan(), buf);
        }
        Ok(())
    }

    fn print_link(&self, indent: usize) -> io::Result<()> {
        writeln!(
            io::stdout().lock(),
            "\n{}\n{:>w$}  {} {}",
            "Source URL:".blue(),
            "",
            "*".blue(),
            self.url.blue(),
            w = indent,
        )
    }
}
