use std::io::{Cursor, Read};

use anyhow::{ensure, Context as _};

pub fn play_audio(url: &str) -> anyhow::Result<()> {
    let resp = ureq::get(url).call()?;
    let len: usize = resp
        .header("Content-Length")
        .context("Content-Length not found")?
        .parse()?;

    ensure!(len <= 16 * 1024 * 1024, "audio file too large: {len} bytes",);

    let mut bytes = Vec::with_capacity(len);

    resp.into_reader()
        .take(len as u64)
        .read_to_end(&mut bytes)?;

    let cursor = Cursor::new(bytes);
    let source = rodio::Decoder::new(cursor)?;
    let (_stream, handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&handle)?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
