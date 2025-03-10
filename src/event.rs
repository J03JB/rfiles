use std::time::Duration;

use futures::{FutureExt, StreamExt};
use crossterm::event::{Event as CrosstermEvent, KeyEvent, EventStream, MouseEvent};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    pub sender: mpsc::UnboundedSender<Event>,
    pub receiver: mpsc::UnboundedReceiver<Event>,
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let t_rate = Duration::from_millis(250);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut t = tokio::time::interval(t_rate);
            loop {
                let crossterm_event = reader.next().fuse();
                let t_delay = t.tick();
                tokio::select! {
                _ = _sender.closed() => {
                    break;
                    }
                    _ = t_delay =>{
                            _sender.send(Event::Tick).unwrap();
                    }
                Some(Ok(evt)) = crossterm_event => {
                    match evt {
                            CrosstermEvent::Key(key) => {
                                if key.kind == crossterm::event::KeyEventKind::Press {
                                    _sender.send(Event::Key(key)).unwrap();
                                }
                            },
                            CrosstermEvent::Resize(x,y) => {
                                _sender.send(Event::Resize(x, y)).unwrap();
                            },
                            _ => {}
                        }
                    }
                };
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> std::result::Result<Event, Box<dyn std::error::Error>> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an Error",
            )))
    }
}
