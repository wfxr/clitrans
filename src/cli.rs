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
}
