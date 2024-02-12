use std::io::{stdout, Stdout};

use crate::event::{EventHandler, Event};
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal, widgets::Paragraph};

use super::Data;
use crate::scene::Scene;

pub struct App {
	pub active: bool,
	pub data: Data,
	// pub backend: CrosstermBackend<Stdout>,
	pub start_time: std::time::Instant,
	pub tick_rate: std::time::Duration,
	pub render_rate: std::time::Duration,
	pub tick_count: u64,
	pub render_count: u64,
	pub scenes: Vec<Scene>,
}

impl App {
	pub fn new(data: Data, tick_rate: u64, render_rate: u64) -> Self {
		Self {
			active: true,
			data: data,
            // unwrap? whats that?
			// backend: CrosstermBackend::new(stdout()),
			start_time: std::time::Instant::now(),
			tick_rate: std::time::Duration::from_millis(tick_rate),
			render_rate: std::time::Duration::from_millis(render_rate),
			tick_count: 0,
			render_count: 0,
			scenes: vec![],
		}
	}

	pub async fn start(&mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
		let mut events = EventHandler::new(self.tick_rate, self.render_rate);

		while self.active {
            let event = events.next().await?;
            match event {
                Event::Error => panic!(),
                Event::Render | Event::Resize(_, _) => {
                    self.render_count += 1;
                    // creating a new terminal every render seems bad
                    let mut draw_result = Ok(());
                    terminal.draw(|f| {
                        // render base
                        f.render_widget(Paragraph::new(format!("ticks: {}, renders: {}", self.tick_count, self.render_count)), f.size());

                        // render top scene (call top scene draw fn)
                        if let Some(scene) = self.scenes.last_mut() {
                            scene.incr_draw();
                            draw_result = (scene.draw)(self, f);
                        }
                    })?;
                    draw_result?
                },
                Event::Tick => {
                    self.tick_count += 1;
                    if let Some(scene) = self.scenes.last_mut() {
                        scene.incr_tick();
                    }
                },
                Event::Key(k) => {
                    // check against global keybinds
                    // TODO: figure out global keybinds
                    if k == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
                        self.active = false
                    }

                    // forward to top scenes
                    if !self.scenes.is_empty() {
                        (self.scenes.last().unwrap().update)(self, event)?;
                    }
                },
                Event::Mouse(_) | Event::Paste(_) => {
                    // forward to top scene
                    if !self.scenes.is_empty() {
                        (self.scenes.last().unwrap().update)(self, event)?;
                    }
                },
            }
        }

		Ok(())
	}
}

impl Default for App {
    fn default() -> Self {
        App::new(Data {}, 50, 50)
    }
}

