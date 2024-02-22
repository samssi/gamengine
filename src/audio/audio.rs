use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub fn play_audio() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("assets/audio/some.wav").unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}