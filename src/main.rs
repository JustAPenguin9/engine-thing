use std::io::stdout;

use color_eyre::eyre::Result;
use crossterm::execute;

mod app;
mod event;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
	let mut app = App::new();

	setup()?;
	let status = app.start().await;
	teardown()?;
	status?;

	Ok(())
}

fn setup() -> Result<()> {
	execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
	crossterm::terminal::enable_raw_mode()?;
	Ok(())
}

fn teardown() -> Result<()> {
	execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
	crossterm::terminal::disable_raw_mode()?;
	Ok(())
}
