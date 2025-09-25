use std::{fs::File, io::BufReader, path::PathBuf, sync::mpsc::Receiver};

use rodio::mixer::Mixer;

pub struct AudioPlayer<'a> {
    path_receiver: Receiver<PathBuf>,
    mixer: &'a Mixer,
}

impl<'a> AudioPlayer<'a> {
    pub fn new(path_receiver: Receiver<PathBuf>, mixer: &'a Mixer) -> Self {
        AudioPlayer {
            path_receiver,
            mixer,
        }
    }

    pub fn run(&mut self) {
        while let Ok(path) = self.path_receiver.recv() {
            let input = BufReader::new(File::open(path).unwrap());
            let sink = rodio::play(self.mixer, input).unwrap();
            sink.sleep_until_end();
        }
    }
}
