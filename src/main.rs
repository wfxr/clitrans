mod cli;
mod engine;
mod translation;
mod util;

use anyhow::{bail, Context as _, Result};
use clap::{CommandFactory as _, Parser as _, ValueEnum};
use clap_complete::{generate, generate_to, Shell};
use cli::{Engine, Opt, SubCommand};
use engine::*;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{collections::HashSet, io, process, sync::mpsc, thread};
use translation::Translation;
use util::build;

pub trait Translate: Send + Clone {
    fn translate(&self, text: &str) -> Result<Option<Translation>>;
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub explanations: usize,
    pub phonetics:    usize,
    pub phrases:      usize,
}

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
    let opts: Opt = Opt::parse();
    match opts.subcommand {
        Some(SubCommand::Completions { shell, dir, list }) => {
            let cmd = &mut Opt::command();
            if list {
                for shell in Shell::value_variants() {
                    println!("{}", shell);
                }
            } else {
                let shell = shell.context("shell not specified")?;
                match dir {
                    Some(dir) => {
                        generate_to(shell, cmd, cmd.get_name().to_string(), dir)?;
                    }
                    None => generate(
                        shell,
                        cmd,
                        cmd.get_name().to_string(),
                        &mut std::io::stdout(),
                    ),
                }
            }
        }
        Some(SubCommand::Version) => {
            println!("{} {}", build::CRATE_NAME, build::CRATE_VERBOSE_VERSION);
        }
        None => {
            let layout = Layout {
                explanations: opts.explanations,
                phrases:      opts.phrases,
                phonetics:    opts.phonetics,
            };
            match &opts.query {
                Some(query) => translate(query, &opts, &layout)?,
                None => loop {
                    let mut rl = DefaultEditor::new()?;
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

fn translate(query: &str, opts: &Opt, layout: &Layout) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let engines: HashSet<_> = opts.engines.iter().cloned().collect();
    let n = engines.len();
    for (id, engine) in engines.into_iter().enumerate() {
        let tx = tx.clone();
        let query = query.to_string();
        thread::spawn(move || {
            let trans = match engine {
                Engine::Bing => bing::Translator.translate(&query),
                Engine::Youdao => youdao::Translator.translate(&query),
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
                print(trans, opts, layout)?;
                break;
            }
        }
    }
    Ok(())
}

fn print(trans: Translation, opts: &Opt, layout: &Layout) -> Result<()> {
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
