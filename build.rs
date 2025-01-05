use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{
    env,
    fs,
    io,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process,
};
use xshell::cmd;

use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    print_git_envs().context("failed to fetch Git information")?;
    print_rustc_envs().context("failed to fetch rustc information")?;
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET")?);

    gen_tests()?;
    Ok(())
}

/// Nicely format an error message for when the subprocess didn't exit
/// successfully.
fn format_error_msg(cmd: &process::Command, output: &process::Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut msg = format!(
        "subprocess didn't exit successfully `{:?}` ({})",
        cmd, output.status
    );
    if !stdout.trim().is_empty() {
        msg.push_str("\n--- stdout\n");
        msg.push_str(&stdout);
    }
    if !stderr.trim().is_empty() {
        msg.push_str("\n--- stderr\n");
        msg.push_str(&stderr);
    }
    msg
}

/// Whether underlying error kind for the given error is
/// `io::ErrorKind::NotFound`.
fn is_io_not_found(error: &anyhow::Error) -> bool {
    for cause in error.chain() {
        if let Some(io_error) = cause.downcast_ref::<io::Error>() {
            return io_error.kind() == io::ErrorKind::NotFound;
        }
    }
    false
}

trait CommandExt {
    /// Run the command and return the standard output as a string.
    fn output_text(&mut self) -> Result<String>;
}

impl CommandExt for process::Command {
    /// Run the command and return the standard output as a string.
    fn output_text(&mut self) -> Result<String> {
        let output = self
            .output()
            .with_context(|| format!("could not execute subprocess: `{self:?}`"))?;
        if !output.status.success() {
            bail!(format_error_msg(self, &output));
        }
        String::from_utf8(output.stdout).context("failed to parse stdout")
    }
}

/// Run a Git subcommand and set the result as a rustc environment variable.
///
/// Note: Success is returned if the Git subcommand is not available.
fn print_git_env(dir: &Path, key: &str, cmd: &str) -> Result<()> {
    let mut split = cmd.split_whitespace();
    let value = match process::Command::new(split.next().unwrap())
        .arg("-C")
        .arg(dir)
        .args(split)
        .output_text()
    {
        Ok(text) => text.trim().to_string(),
        Err(err) if is_io_not_found(&err) => return Ok(()),
        Err(err) => return Err(err),
    };
    println!("cargo:rustc-env={key}={value}");
    Ok(())
}

/// Fetch Git info and set as rustc environment variables.
///
/// If the Git subcommand is missing or the `.git` directory does not exist then
/// no errors will be produced.
fn print_git_envs() -> Result<()> {
    let dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    if !dir.join(".git").exists() {
        return Ok(());
    }
    print_git_env(
        &dir,
        "GIT_COMMIT_DATE",
        "git log -1 --date=short --format=%cd",
    )?;
    print_git_env(&dir, "GIT_COMMIT_HASH", "git rev-parse HEAD")?;
    print_git_env(
        &dir,
        "GIT_COMMIT_SHORT_HASH",
        "git rev-parse --short=9 HEAD",
    )?;
    Ok(())
}

/// Fetch rustc info and set as rustc environment variables.
fn print_rustc_envs() -> Result<()> {
    let text = process::Command::new(env::var("RUSTC")?)
        .arg("--verbose")
        .arg("--version")
        .output_text()?;
    let mut lines = text.lines();
    println!(
        "cargo:rustc-env=RUSTC_VERSION_SUMMARY={}",
        lines.next().unwrap()
    );
    for line in lines {
        let (key, value) = line.split_once(": ").unwrap();
        println!(
            "cargo:rustc-env=RUSTC_VERSION_{}={}",
            key.replace(['-', ' '], "_").to_uppercase(),
            value,
        );
    }
    Ok(())
}

// Stealing from https://github.com/rust-analyzer/rust-analyzer/blob/5f6d71cf/xtask/src/codegen.rs#L74
fn reformat(text: &str) -> Result<String> {
    let sh = xshell::Shell::new()?;
    let rustfmt_toml = "rustfmt.toml";
    let stdout = cmd!(sh, "rustfmt --config-path {rustfmt_toml}")
        .stdin(text)
        .read()?;
    let preamble = "Generated file, do not edit by hand, see `/build.rs`";
    Ok(format!(
        "//! {}\n\n{}\n\n{}\n",
        preamble, stdout, "// vim: ro"
    ))
}

fn gen_tests() -> Result<()> {
    for key in &["bing", "youdao"] {
        let test_data = fs::read_to_string(format!("./src/engine/{}/test_data.json", key))?;
        let test_data: Vec<TestData> = serde_json::from_str(&test_data)?;
        if !test_data.is_empty() {
            let mut buf = BufWriter::new(Vec::new());
            let key = format_ident!("{}", key);
            writeln!(&mut buf, "{}", quote! {
                use super::Translator;
                use crate::{{Translate, Translation}};
            })?;
            for item in test_data.into_iter() {
                let name = format_ident!("{}", item.name);
                let input = item.input;
                let expect = item.expect.to_string();
                let test_fn = quote! {
                    #[test]
                    fn #name() {
                        let trans = Translator;
                        let r = trans.translate(#input);
                        assert!(r.is_ok());
                        let r = r.unwrap();
                        assert!(r.is_some());
                        let r = r.unwrap();

                        let expected : Translation = serde_json::from_str(r#"{#expect}"#).unwrap();
                        let msg = format!("\n=== Json dump:\n{}\n=== ", serde_json::to_string(&r).unwrap());
                        assert_eq!(r.query, expected.query, "{}", msg);
                        assert_eq!(r.url, expected.url, "{}", msg);
                        assert_eq!(r.exps, expected.exps, "{}", msg);
                        assert_eq!(r.phrases, expected.phrases, "{}", msg);
                    }
                };
                let test_fn = test_fn.to_string().replace("{#expect}", &expect);
                writeln!(&mut buf, "{}", test_fn)?;
            }
            let text = reformat(&String::from_utf8(buf.into_inner()?)?)?;
            fs::write(format!("./src/engine/{}/test.rs", key), text)?;
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TestData<'a> {
    name:   String,
    input:  String,
    #[serde(borrow)]
    expect: &'a RawValue,
}
