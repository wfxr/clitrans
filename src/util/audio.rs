use std::io::Cursor;

pub async fn play_audio(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let cursor = Cursor::new(resp.bytes().await?);
    let source = rodio::Decoder::new(cursor).unwrap();
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
