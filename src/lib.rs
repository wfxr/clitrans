pub mod bing;
mod util;
pub mod youdao;

pub use async_trait::async_trait;
use colored::{Color, Colorize};

#[async_trait]
pub trait Translate {
    async fn translate(&self, text: &str) -> Result<Option<Translation>, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct Translation {
    query: String,
    prons: Vec<Pronunciation>,
    exps:  Vec<Explanation>,
    mexp:  Option<String>,
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
    pos:    String,
    values: Vec<String>,
}

impl Translation {
    pub fn print(&self) {
        print!("{}: ", self.query.bold().underline());
        if !self.prons.is_empty() {
            for pron in &self.prons {
                print!("{}[{}] ", pron.region, pron.value.yellow().bold())
            }
        }
        println!();

        println!();
        let pos_width = self.exps.iter().map(|exp| exp.pos.len()).max().unwrap_or(0);
        for exp in &self.exps {
            let color = match exp.pos.as_str() {
                "Web." => Color::Magenta,
                _ => Color::Green,
            };
            for (i, item) in exp.values.iter().enumerate() {
                let title = if i == 0 { &exp.pos } else { "" };
                println!(
                    "{:>width$}  {} {}",
                    title.italic().color(color),
                    "*".color(color),
                    item.color(color),
                    width = pos_width + 1
                )
            }
            println!()
        }

        if let Some(exp) = &self.mexp {
            println!("Machine Translation");
            println!("{}", exp.cyan());
        }
    }
}
