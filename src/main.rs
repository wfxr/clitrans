mod cli;

use anyhow::anyhow;
use cli::*;
use clitrans::{engine::*, Layout, Translate};
use std::{
    io::{self, stdout, Write},
    process,
};

fn main() {
    if let Err(e) = try_main() {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        eprintln!();
        eprintln!("Messages:");
        eprintln!("  * {}", e);
        process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let opts: Opts = Opts::from_args();
    match opts.subcommand {
        Some(Subcommand::Completion(CompletionOpt { shell })) => {
            Opts::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut stdout());
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
                    match engine.translate(&$query)? {
                        Some(trans) => {
                            trans.print(&layout);
                            #[allow(unused_variables)]
                            if let Some(tag) = &opts.audio {
                                #[cfg(feature = "audio")]
                                trans.play_audio(tag)?;
                                #[cfg(not(feature = "audio"))]
                                return Err(anyhow!("audio is not enabled"));
                            }
                        }
                        None => return Err(anyhow!("translation not found")),
                    }
                };
            }

            match opts.query {
                Some(query) => translate!(query),
                None => loop {
                    print!("> ");
                    std::io::stdout().flush()?;
                    let mut query = String::new();
                    if std::io::stdin().read_line(&mut query)? == 0 {
                        println!();
                        break;
                    };
                    if query.trim().is_empty() {
                        continue;
                    }
                    translate!(query)
                },
            }
        }
    }
    Ok(())
}
