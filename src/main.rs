use std::{env::current_dir, path::PathBuf, process::Command};

use clipboard_stream::{ClipboardEventListener, Kind};
use rodio::OutputStreamBuilder;
use soiyomi::{
    audio_file_creator::{AudioCreate, AudioFileCreator},
    audio_player::AudioPlayer,
    event_sender::EventSender,
};
use thiserror::Error;

fn main() {
    let mut clipboard_event = ClipboardEventListener::spawn();
    let clipboard_stream = clipboard_event.new_stream(Kind::Utf8String, 32).unwrap();

    let (text_tx, text_rx) = std::sync::mpsc::channel();

    let mut event_sender = EventSender::new(clipboard_stream, text_tx);

    let (tx, rx) = std::sync::mpsc::channel();
    let prosess = Prosess {
        prosess_cfg: ProsessConfig {
            path: "/Applications/voicepeak.app/Contents/MacOS/voicepeak".into(),
            narrator: "Asumi Ririse".to_string(),
        },
        count: 0,
    };

    let mut audio_file_creator = AudioFileCreator::new(prosess, text_rx, tx);

    std::thread::spawn(move || audio_file_creator.run());

    std::thread::spawn(move || {
        let _ = futures::executor::block_on(event_sender.run());
    });

    let stream = OutputStreamBuilder::open_default_stream().unwrap();
    let mixer = stream.mixer();
    let mut player = AudioPlayer::new(rx, mixer);
    // this method blocking current thread
    player.run();
}

pub struct Prosess {
    prosess_cfg: ProsessConfig,
    count: u32,
}

#[derive(Debug, Clone, Error)]
#[error("err")]
pub struct MyErr;

impl AudioCreate for Prosess {
    type Error = MyErr;

    fn create(&mut self, text: impl Into<String>) -> Result<PathBuf, Self::Error> {
        let file_name = format!("hello.wav");
        let mut child = Command::new(self.prosess_cfg.path.clone())
            .arg("-s")
            .arg(text.into())
            .arg("-n")
            .arg(self.prosess_cfg.narrator.clone())
            .arg("-o")
            .arg(file_name)
            .spawn()
            .map_err(|_| MyErr)?;

        let status = child.wait().unwrap();

        if !status.success() {
            return Err(MyErr);
        }

        let mut c = current_dir().unwrap().into_os_string();
        let f = format!("/test-{}.wav", self.count);
        c.push(f);

        let mut a = PathBuf::new();
        a.push(c);

        self.count += 1;
        Ok("hello.wav".into())
    }
}

pub struct ProsessConfig {
    path: PathBuf,
    narrator: String,
}
