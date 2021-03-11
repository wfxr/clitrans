#![feature(async_closure)]
mod cli;

use std::{io::Write, process};

use cli::*;
use clitrans::{engine::*, Layout, Translate};

#[tokio::main]
async fn main() {
    if let Err(e) = async || -> Result<(), Box<dyn std::error::Error>> {
        let opts: Opts = Opts::from_args();
        match opts.subcommand {
            Some(Subcommand::Completion(CompletionOpt { shell })) => {
                Opts::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
            }
            None => {
                let engine: Box<dyn Translate> = match opts.engine {
                    Engine::bing => Box::new(bing::Translator),
                    Engine::youdao => Box::new(youdao::Translator),
                };

                let layout = Layout {
                    explanations: opts.explanations,
                    phrases:      opts.phrases,
                    phonetics:    opts.phonetics,
                };

                macro_rules! translate {
                    ($query:expr) => {
                        match engine.translate(&$query).await? {
                            Some(trans) => {
                                trans.print(&layout);
                                #[cfg(feature = "audio")]
                                if let Some(tag) = &opts.audio {
                                    trans.play_audio(tag).await?;
                                }
                            }
                            None => return Err("translation not found".into()),
                        }
                    };
                }

                match opts.query {
                    Some(query) => {
                        translate!(query);
                    }
                    None => loop {
                        print!("> ");
                        std::io::stdout().flush()?;
                        let mut query = String::new();
                        std::io::stdin().read_line(&mut query)?;
                        translate!(query);
                    },
                }
            }
        }
        Ok(())
    }()
    .await
    {
        eprintln!();
        eprintln!("Messages:");
        eprintln!("  * {}", e);
        process::exit(1);
    }
}
