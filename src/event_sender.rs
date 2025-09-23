use clipboard_stream::{Body, ClipboardStream};
use futures::StreamExt;

/// Clipboard change event sender to AudioFileCreater
///
/// receive clipbard change event as Body and send String to AudioFileCreater.
pub struct EventSender {
    clipboard_stream: ClipboardStream,
    text_sender: std::sync::mpsc::Sender<String>,
}

impl EventSender {
    pub fn new(
        clipboard_stream: ClipboardStream,
        text_sender: std::sync::mpsc::Sender<String>,
    ) -> Self {
        EventSender {
            clipboard_stream,
            text_sender,
        }
    }

    pub async fn run(&mut self) {
        while let Some(body_result) = self.clipboard_stream.next().await {
            let text = match body_result {
                Ok(v) => match v {
                    Body::Utf8String(text) => text,
                },
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            if let Err(e) = self.text_sender.send(text) {
                eprintln!("{}", e);
            }
        }
    }
}
