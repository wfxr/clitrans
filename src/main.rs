mod cli;

use anyhow::{bail, Result};
use cli::*;
use clitrans::{engine::*, Layout, Translate, Translation};
use rustyline::{error::ReadlineError, Editor};
use std::sync::mpsc;
use std::{
    collections::HashSet,
    io::{self, stdout},
    process, thread,
};

fn main() {
    match try_main() {
        Err(e) => {
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
        Ok(code) => process::exit(code),
    }
}

fn try_main() -> Result<i32> {
    let opts: Opts = Opts::from_args();
    match opts.subcommand {
        Some(Subcommand::Completion(CompletionOpt { shell })) => {
            Opts::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut stdout());
        }
        None => {
            let layout = Layout {
                explanations: opts.explanations,
                phrases:      opts.phrases,
                phonetics:    opts.phonetics,
            };
            match &opts.query {
                Some(query) => translate(&query, &opts, &layout)?,
                None => loop {
                    let mut rl = Editor::<()>::new();
                    let line = rl.readline("> ");
                    match line {
                        Ok(query) => {
                            if query.trim().is_empty() {
                                continue;
                            }
                            translate(&query, &opts, &layout)?
                        }
                        Err(ReadlineError::Eof) => break,
                        Err(ReadlineError::Interrupted) => return Ok(1),
                        Err(e) => bail!(e),
                    }
                },
            }
        }
    }
    Ok(0)
}

fn translate(query: &str, opts: &Opts, layout: &Layout) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let engines: HashSet<_> = opts.engines.iter().cloned().collect();
    let n = engines.len();
    for (id, engine) in engines.into_iter().enumerate() {
        let tx = tx.clone();
        let query = query.to_string();
        thread::spawn(move || {
            let trans = match engine {
                Engine::bing => bing::Translator.translate(&query),
                Engine::youdao => youdao::Translator.translate(&query),
            };
            tx.send((n - id, trans)) // ignore errors since the receiver may be deallocated
        });
    }
    loop {
        let (id, trans) = rx.recv().expect("failed receiving translation");
        match (id, trans) {
            (1, Err(e)) => return Err(e),
            (1, Ok(None)) => bail!("translation not found"),
            (_, Err(_)) | (_, Ok(None)) => continue,
            (_, Ok(Some(trans))) => {
                print(trans, &opts, &layout)?;
                break;
            }
        }
    }
    Ok(())
}

fn print(trans: Translation, opts: &Opts, layout: &Layout) -> Result<()> {
    trans.print(layout)?;
    match &opts.audio {
        Some(_tag) => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "audio")] {
                    trans.play_audio(_tag)
                } else {
                    bail!("audio is not enabled")
                }
            }
        }
        None => Ok(()),
    }
}
