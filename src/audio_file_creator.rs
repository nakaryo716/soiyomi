use std::{
    error::Error,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
};

pub trait AudioCreate {
    type Error: Error;

    fn create(&mut self, text: impl Into<String>) -> Result<PathBuf, Self::Error>;
}

pub struct AudioFileCreator<T> {
    audio_prosess: T,
    text_receiver: Receiver<String>,
    create_notify: Sender<PathBuf>,
}

impl<T> AudioFileCreator<T>
where
    T: AudioCreate + 'static,
{
    pub fn new(
        audio_prosess: T,
        text_receiver: Receiver<String>,
        create_notify: Sender<PathBuf>,
    ) -> Self {
        AudioFileCreator {
            audio_prosess,
            text_receiver,
            create_notify,
        }
    }

    pub fn run(&mut self) {
        while let Ok(text) = self.text_receiver.recv() {
            println!("{}", text);
            let file_path = match self.audio_prosess.create(text) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            if let Err(e) = self.create_notify.send(file_path) {
                eprintln!("{}", e);
            }
        }
    }
}
