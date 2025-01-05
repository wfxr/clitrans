use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
    str::FromStr,
};
use structopt::clap::Shell;
use xshell::cmd;

include!("src/cli.rs");

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    gen_completions()?;
    gen_tests()?;
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

fn gen_completions() -> Result<()> {
    let outdir = std::env::var_os("SHELL_COMPLETIONS_DIR")
        .or_else(|| std::env::var_os("OUT_DIR"))
        .expect("OUT_DIR not found");
    let outdir_path = Path::new(&outdir);
    let mut app = Opts::clap();

    for shell in &Shell::variants() {
        let dir = outdir_path.join(shell);
        fs::create_dir_all(&dir)?;
        app.gen_completions(env!("CARGO_PKG_NAME"), Shell::from_str(shell)?, &dir);
    }
    Ok(())
}

fn gen_tests() -> Result<()> {
    for key in &["bing", "youdao"] {
        let test_data = fs::read_to_string(format!("./tests/engine/{}/test_data.json", key))?;
        let test_data: Vec<TestData> = serde_json::from_str(&test_data)?;
        if !test_data.is_empty() {
            let mut buf = BufWriter::new(Vec::new());
            let key = format_ident!("{}", key);
            writeln!(&mut buf, "{}", quote! {
                use clitrans::engine::#key::Translator;
                use clitrans::{{Translate, Translation}};
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
            fs::write(format!("./tests/engine/{}/mod.rs", key), text)?;
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
