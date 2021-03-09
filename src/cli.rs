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
    pub query:   String,
    /// Translate engine
    #[clap(short, long, arg_enum, default_value = "bing", case_insensitive = true)]
    pub engine:  Engine,
    /// How many web phrases to display
    #[clap(short, long, default_value = "3")]
    pub phrases: usize,
}

#[derive(Clap, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Engine {
    youdao,
    bing,
}
