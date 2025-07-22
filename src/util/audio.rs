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

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .context("Failed to open default audio stream")?;
    let input = Cursor::new(bytes);
    let sink = rodio::play(stream_handle.mixer(), input).context("Failed to play audio")?;
    sink.sleep_until_end();
    Ok(())
}
