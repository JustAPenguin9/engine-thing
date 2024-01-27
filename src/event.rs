use color_eyre::eyre::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::{
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinHandle,
};

pub enum Event {
	Error,
	Tick,
	Render,
	Key(KeyEvent),
	Mouse(MouseEvent),
	Paste(String),
	Resize(u16, u16),
}

pub struct EventHandler {
	pub handler: JoinHandle<()>,
	pub receiver: UnboundedReceiver<Event>,
	pub sender: UnboundedSender<Event>,
	pub tick_rate: u64,
	pub frame_rate: u64,
}

impl EventHandler {
	pub fn new(tick_rate: u64, frame_rate: u64) -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
		EventHandler {
			handler: tokio::spawn(async {}),
			receiver: receiver,
			sender: sender,
			tick_rate: tick_rate,
			frame_rate: frame_rate,
		}
	}

	pub fn start(&mut self) {
		let _sender = self.sender.clone();
		let tick_delay = std::time::Duration::from_millis(self.tick_rate);
		let render_delay = std::time::Duration::from_millis(self.frame_rate);
		self.handler = tokio::spawn(async move {
			let mut reader = crossterm::event::EventStream::new();
			let mut tick_interval = tokio::time::interval(tick_delay);
			let mut render_interval = tokio::time::interval(render_delay);
			loop {
				let tick_delay = tick_interval.tick();
				let render_delay = render_interval.tick();
				let crossterm_event = reader.next().fuse();
				tokio::select! {
				  maybe_event = crossterm_event => {
					match maybe_event {
					  Some(Ok(evt)) => {
						match evt {
							CrosstermEvent::Key(key) => {
								if key.kind == KeyEventKind::Press {
									_sender.send(Event::Key(key)).unwrap();
								}
							},
							CrosstermEvent::Mouse(mouse) => {
								_sender.send(Event::Mouse(mouse)).unwrap();
							},
							CrosstermEvent::Resize(x, y) => {
								_sender.send(Event::Resize(x, y)).unwrap();
							},
							CrosstermEvent::Paste(s) => {
								// doesnt work on windows?
								_sender.send(Event::Paste(s)).unwrap();
							},
							_ => {}, // focus gained and lost not needed
						}
						}
						Some(Err(_)) => {
							// im not sure if this even works
							_sender.send(Event::Error).unwrap();
						}
						None => {},
					}
				  },
				  _ = tick_delay => {
					  _sender.send(Event::Tick).unwrap();
				  },
				  _ = render_delay => {
					  _sender.send(Event::Render).unwrap();
				  },
				}
			}
		});
	}

	pub async fn next(&mut self) -> Result<Event> {
		self.receiver.recv().await.ok_or(color_eyre::eyre::eyre!("Unable to get event"))
	}
}
