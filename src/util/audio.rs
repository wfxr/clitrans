use isahc::ReadResponseExt;
use std::io::Cursor;

pub fn play_audio(url: &str) -> anyhow::Result<()> {
    let mut resp = isahc::get(url)?;
    let mut buf = vec![];
    resp.copy_to(&mut buf)?;
    let cursor = Cursor::new(buf);
    let source = rodio::Decoder::new(cursor)?;
    let (_stream, handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&handle)?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
