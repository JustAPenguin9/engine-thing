use std::io::stdout;

use color_eyre::eyre::Result;
use crate::event::*;
use ratatui::prelude::*;
use ratatui::widgets::*;
use crossterm::event::KeyCode::*;

pub struct App {
	pub active: bool,
	pub frame: usize,
}

impl Default for App {
	fn default() -> Self {
		Self { active: true, frame: 0 }
	}
}

impl App {
	pub fn new() -> Self {
		Self::default()
	}

	pub async fn start(&mut self) -> Result<()> {
        let mut events = EventHandler::new(25);
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            let event = events.next().await?;

            self.update(event)?;

            terminal.draw(|f| {
                self.ui(f);
            })?;

            if !self.active {
                break;
            }
        }

		Ok(())
	}

    fn update(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => {
                match key.code {
                    Char('q') => self.active = false,
                    _ => {},
                }
            },
            // TODO: these
            Event::Error => {},
            Event::Tick => {},
            Event::Render => {},
        }

        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        f.render_widget(Paragraph::new(format!("{}", self.frame)), f.size());
    }
}
