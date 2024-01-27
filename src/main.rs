use std::io::stdout;

use color_eyre::eyre::Result;
use crossterm::execute;

mod app;
mod data;
mod event;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
	let mut app = App::new()?;

	println!("welcome");
	setup()?;
	// 50 => 20 fps
	// 25 => 40 tps
	let status = app.start(25, 50).await;
	teardown()?;
	println!("bye bye");
	status?;

	Ok(())
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
