use std::path::PathBuf;

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser,
    ValueEnum,
};
use clap_complete::Shell;

use crate::util::build;

#[derive(Debug, PartialEq, Eq, Parser)]
#[clap(author, about)]
#[clap(version = build::CRATE_VERSION)]
#[clap(long_version = build::CRATE_LONG_VERSION)]
#[clap(
    styles(Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Green.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
    )
)]
pub struct Opt {
    /// Text to translate
    #[clap(name = "QUERY")]
    pub query: Option<String>,

    /// Translate engine
    #[clap(
        short,
        long,
        value_enum,
        value_delimiter = ',',
        env = "CLITRANS_ENGINES",
        default_values_t = vec![Engine::Youdao, Engine::Bing]
    )]
    pub engines: Vec<Engine>,

    /// How many explanations to display
    #[clap(long, env = "CLITRANS_EXPLANATIONS", default_value = "20")]
    pub explanations: usize,

    /// How many phonetics to display
    #[clap(long, env = "CLITRANS_PHONETICS", default_value = "2")]
    pub phonetics: usize,

    /// How many web phrases to display
    #[clap(short, long, env = "CLITRANS_PHRASES", default_value = "3")]
    pub phrases: usize,

    /// Play pronounce audio (if any)
    #[clap(short, long, env = "CLITRANS_AUDIO")]
    pub audio: Option<String>,

    /// Subcommand
    #[clap(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Debug, PartialEq, Eq, Parser)]
pub enum SubCommand {
    /// Generate completions for the given shell.
    Completions {
        /// The shell to generate completions for.
        #[clap(value_name = "SHELL", value_enum, required_unless_present = "list")]
        shell: Option<Shell>,

        /// The directory to write the completions to.
        ///
        /// Defaults output to stdout.
        #[clap(short, long, value_name = "DIR")]
        dir: Option<PathBuf>,

        /// List all available shells.
        #[clap(short, long, exclusive = true)]
        list: bool,
    },

    /// Prints detailed version information.
    Version,
}

#[derive(Debug, PartialEq, Eq, Parser)]
pub struct CompletionOpt {
    /// Target shell name
    #[clap(value_enum)]
    pub shell: Shell,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Hash, Clone)]
pub enum Engine {
    Youdao,
    Bing,
}
