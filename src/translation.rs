use super::Layout;
use colored::{Color, Colorize};
use itertools::Itertools;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Pronunciation {
    pub region: &'static str,
    pub value:  String,
}

impl Pronunciation {
    pub fn pinyin(value: String) -> Self {
        Self { region: "CN", value }
    }
    pub fn us(value: String) -> Self {
        Self { region: "US", value }
    }
    pub fn uk(value: String) -> Self {
        Self { region: "UK", value }
    }
}

#[derive(Debug, Clone)]
pub struct Explanation {
    pub tag:   ExpTag,
    pub items: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ExpTag {
    Web,
    Machine,
    Phrase,
    Pos(String),
}

impl Translation {
    pub fn print(&self, layout: &Layout) {
        self.print_query();
        self.print_pronunciations();

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

        self.print_explanations(indent, &exps);
        self.print_phrases(&layout, indent);
        self.print_link(indent);
    }

    fn print_query(&self) {
        println!("{}", self.query);
    }

    fn print_pronunciations(&self) {
        if !self.prons.is_empty() {
            let buf = self
                .prons
                .iter()
                .map(|pron| &pron.value)
                .unique()
                .map(|s| format!("/{}/", s.yellow()))
                .join(", ");
            println!("{}", buf);
        }
    }

    fn print_explanations(&self, indent: usize, exps: &[(Color, &str, &[String])]) {
        if !exps.is_empty() {
            let buf = exps
                .iter()
                .map(|(color, tag, itmes)| {
                    itmes
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            let title = if i == 0 { tag } else { "" };
                            format!(
                                "{:>indent$}  {} {}",
                                title.color(*color).italic(),
                                "*".color(*color),
                                item.color(*color)
                            )
                        })
                        .join("\n")
                })
                .join("\n\n");
            println!("\n{}", buf);
        }
    }

    fn print_phrases(&self, layout: &Layout, indent: usize) {
        if !self.phrases.is_empty() && layout.phrases > 0 {
            let buf = self
                .phrases
                .iter()
                .filter(|(_, exps)| !exps.is_empty())
                .take(layout.phrases)
                .map(|(phrase, exps)| {
                    format!("{:>indent$}  {} {}\n", "", "*".cyan(), phrase.cyan())
                        + &exps
                            .iter()
                            .map(|exp| format!("{:>indent$}    {} {}", "", "-".cyan(), exp.cyan()))
                            .join("\n")
                })
                .join("\n\n");
            println!("\n{}\n{}", "Web Phrases:".cyan(), buf);
        }
    }

    fn print_link(&self, indent: usize) {
        println!(
            "\n{}\n{:>indent$}  {} {}",
            "Source URL:".blue(),
            "",
            "*".blue(),
            self.url.blue()
        );
    }
}
