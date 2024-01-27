use crossterm::event::KeyEvent;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use color_eyre::eyre::Result;
use futures::{FutureExt, StreamExt};

pub enum Event {
	Error,
	Tick,
    Render,
	Key(KeyEvent),
}

pub struct EventHandler {
	_tx: mpsc::UnboundedSender<Event>,
	rx: mpsc::UnboundedReceiver<Event>,
	task: Option<JoinHandle<()>>,
}

impl EventHandler {
	pub fn new(ms: u64) -> Self {
		let tick_rate = std::time::Duration::from_millis(ms);

		let (tx, rx) = mpsc::unbounded_channel();
		let _tx = tx.clone();

		let task = tokio::spawn(async move {
			let mut reader = crossterm::event::EventStream::new();
			let mut interval = tokio::time::interval(tick_rate);
			loop {
				let delay = interval.tick();
				let crossterm_event = reader.next().fuse();
				tokio::select! {
				  maybe_event = crossterm_event => {
					match maybe_event {
					  Some(Ok(evt)) => {
						match evt {
						  crossterm::event::Event::Key(key) => {
							if key.kind == crossterm::event::KeyEventKind::Press {
							  tx.send(Event::Key(key)).unwrap();
							}
						  },
						  _ => {},
						}
					  }
					  Some(Err(_)) => {
						tx.send(Event::Error).unwrap();
					  }
					  None => {},
					}
				  },
				  _ = delay => {
					  tx.send(Event::Tick).unwrap();
				  },
				}
			}
		});

		Self { _tx, rx, task: Some(task) }
	}

	pub async fn next(&mut self) -> Result<Event> {
		self.rx.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
	}
}
