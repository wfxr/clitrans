mod cli;

use anyhow::Result;
use cli::*;
use clitrans::{engine::*, Layout, Translate, Translation};
use std::sync::mpsc;
use std::{
    collections::HashSet,
    io::{self, stdout, Write},
    process, thread,
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

fn try_main() -> Result<()> {
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
                    translate(&query, &opts, &layout)?
                },
            }
        }
    }
    Ok(())
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
            (0, Err(e)) => return Err(e),
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
    trans.print(layout);
    match &opts.audio {
        Some(_tag) => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "audio")] {
                    trans.play_audio(_tag)
                } else {
                    Err(anyhow!("audio is not enabled"))
                }
            }
        }
        None => Ok(()),
    }
}
