pub mod bing;

use colored::{Color, Colorize};

pub trait Parser {
    fn parse(content: &str) -> Translation;
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
    prop:  String,
    value: Vec<String>,
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
        let prop_width = self.exps.iter().map(|exp| exp.prop.len()).max().unwrap_or(0);
        for exp in &self.exps {
            let color = match exp.prop.as_str() {
                "Web." => Color::Magenta,
                _ => Color::Green,
            };
            for (i, item) in exp.value.iter().enumerate() {
                let title = if i == 0 { &exp.prop } else { "" };
                println!(
                    "{:>width$}  {} {}",
                    title.italic().color(color),
                    "*".color(color),
                    item.color(color),
                    width = prop_width + 1
                )
            }
            println!()
        }
    }
}
