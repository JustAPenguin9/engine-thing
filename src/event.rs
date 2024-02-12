use color_eyre::eyre::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::{
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinHandle,
};

/// Error: add new scene  
/// Update: call update
/// Render: call draw
/// Key: call update
/// Mouse: call update
/// Paste: call update
/// Resize: call draw
pub enum Event {
	Error,             // add new scene
	Tick,              // call update
	Render,            // call draw
	Key(KeyEvent),     // call update
	Mouse(MouseEvent), // call update
	Paste(String),     // call update
	Resize(u16, u16),  // call draw
}

// most (all) of this is taken from the ratatui tutorial and async template
// https://github.com/ratatui-org/templates/blob/main/simple-async/src/event.rs
// https://ratatui.rs/tutorials/counter-async-app/
pub struct EventHandler {
	handler: JoinHandle<()>,
	sender: UnboundedSender<Event>,
	receiver: UnboundedReceiver<Event>,
}

impl EventHandler {
	pub fn new(update_rate: std::time::Duration, render_rate: std::time::Duration) -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
        let tx = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut update_interval = tokio::time::interval(update_rate);
            let mut render_interval = tokio::time::interval(render_rate);

            loop {
                let update_delay = update_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                
                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                match event {
                                    CrosstermEvent::Key(key) => {
                                        if key.kind == KeyEventKind::Press {
                                            tx.send(Event::Key(key)).unwrap();
                                        }
                                    },
                                    CrosstermEvent::Mouse(mouse) => {
                                        tx.send(Event::Mouse(mouse)).unwrap();
                                    },
                                    CrosstermEvent::Resize(x, y) => {
                                        tx.send(Event::Resize(x, y)).unwrap();
                                    },
                                    CrosstermEvent::Paste(s) => {
                                        // doesnt work on windows?
                                        tx.send(Event::Paste(s)).unwrap();
                                    },
                                    // focus gained and lost not needed
                                    _ => {},
                                }
                            }
                            Some(Err(_)) => {
                                // TODO: this should also send the error upstream
                                tx.send(Event::Error).unwrap();
                            }
                            None => {},
                        }
                    },
                    _ = update_delay => {
                        tx.send(Event::Tick).unwrap();
                    },
                    _ = render_delay => {
                        tx.send(Event::Render).unwrap();
                    }
                }
            }
        });

		EventHandler {
			handler,
			sender,
			receiver,
		}
	}

	pub async fn next(&mut self) -> Result<Event> {
		self.receiver.recv().await.ok_or(color_eyre::eyre::eyre!("unable to get next event"))
	}
}
