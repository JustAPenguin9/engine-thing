// TODO: remove this eventually
#![allow(dead_code)]

use std::io::stdout;

use color_eyre::eyre::Result;

mod app;
mod event;
mod scene;
use app::App;
use crossterm::execute;
use ratatui::{backend::CrosstermBackend, Terminal};

struct Data {}

#[tokio::main]
async fn main() -> Result<()> {
	let mut app = App { ..Default::default() };
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	setup()?;
	let status = app.start(terminal).await;
	teardown()?;
	println!("bye");
	status
}

fn setup() -> Result<()> {
	execute!(
		stdout(),
		crossterm::terminal::EnterAlternateScreen,
		crossterm::event::EnableMouseCapture,
		// crossterm::event::EnableBracketedPaste,
	)?;
	crossterm::terminal::enable_raw_mode()?;
	Ok(())
}

fn teardown() -> Result<()> {
	execute!(
		stdout(),
		crossterm::terminal::LeaveAlternateScreen,
		crossterm::event::DisableMouseCapture,
		// crossterm::event::DisableBracketedPaste,
	)?;
	crossterm::terminal::disable_raw_mode()?;
	Ok(())
}
