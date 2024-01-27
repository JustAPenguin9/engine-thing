use std::io::stdout;

use crossterm::{
	event::{self, Event, KeyCode},
	execute,
};
use ratatui::{prelude::*, widgets::Paragraph};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

mod app;
use app::App;
// mod event;

fn main() -> Result<()> {
	let app = App { ..Default::default() };

	setup()?;
	let status = start(app);
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

fn start(mut app: App) -> Result<()> {
	let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	// main loop
	loop {
		terminal.draw(|f| {
			app.frame += 1;
			ui(&app, f);
		})?;

		update(&mut app)?;

		if !app.active {
			break;
		}
	}
	Ok(())
}

fn ui(app: &App, f: &mut Frame) {
	f.render_widget(Paragraph::new(format!("{}", app.frame)), f.size());
}

fn update(app: &mut App) -> Result<()> {
	// 50 fps
	if event::poll(std::time::Duration::from_millis(25))? {
		if let Event::Key(key) = event::read()? {
			if key.kind == event::KeyEventKind::Press {
				match key.code {
					KeyCode::Char('q') => app.active = false,
					_ => {}
				}
			}
		}
	}

	Ok(())
}
