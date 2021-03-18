use structopt::clap::{self, arg_enum, AppSettings};
pub use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"),
           about = env!("CARGO_PKG_DESCRIPTION"),
         version = env!("CARGO_PKG_VERSION"),
         global_setting(AppSettings::ColoredHelp),
)]
pub struct Opts {
    /// Text to translate
    #[structopt(name = "QUERY")]
    pub query: Option<String>,

    /// Translate engine
    #[structopt(
        short,
        long,
        env = "CLITRANS_ENGINES",
        default_value = "bing,youdao",
        possible_values = &Engine::variants(),
        case_insensitive = true,
        use_delimiter = true,
        require_delimiter = true,
    )]
    pub engines: Vec<Engine>,

    /// How many explanations to display
    #[structopt(long, env = "CLITRANS_EXPLANATIONS", default_value = "20")]
    pub explanations: usize,

    /// How many phonetics to display
    #[structopt(long, env = "CLITRANS_PHONETICS", default_value = "2")]
    pub phonetics: usize,

    /// How many web phrases to display
    #[structopt(short, long, env = "CLITRANS_PHRASES", default_value = "3")]
    pub phrases: usize,

    /// Play pronounce audio (if any)
    #[structopt(short, long, env = "CLITRANS_AUDIO")]
    pub audio: Option<String>,

    /// Subcommand
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(StructOpt)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion(CompletionOpt),
}

#[derive(StructOpt)]
pub struct CompletionOpt {
    /// Target shell name
    #[structopt(possible_values = &clap::Shell::variants())]
    pub shell: clap::Shell,
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq, Eq, Hash, Clone)]
    pub enum Engine {
        youdao,
        bing,
    }
}
