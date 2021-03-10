use clap::AppSettings;

pub use clap::Clap;

#[derive(Clap, Debug)]
#[clap( name = env!("CARGO_PKG_NAME"),
       about = env!("CARGO_PKG_DESCRIPTION"),
     version = env!("CARGO_PKG_VERSION"),
     global_setting(AppSettings::ColoredHelp),
)]
pub struct Opts {
    /// Text to translate
    #[clap(name = "QUERY")]
    pub query: String,

    /// Translate engine
    #[clap(short, long, arg_enum, default_value = "bing", case_insensitive = true)]
    pub engine: Engine,

    /// How many phonetics to display
    #[clap(long, default_value = "2")]
    pub phonetics: usize,

    /// How many web phrases to display
    #[clap(short, long, default_value = "3")]
    pub phrases: usize,

    /// Play pronounce audio (if any)
    #[clap(short, long)]
    pub audio: Option<String>,
}

#[derive(Clap, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Engine {
    youdao,
    bing,
}
