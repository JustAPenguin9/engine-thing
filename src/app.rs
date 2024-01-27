// TODO: refactor into multiple files
// TODO: ??? refactor into a lib folder

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::Terminal;
use std::io::stdout;
use std::io::Stdout;

use super::data::Data;
use super::event::Event;
use super::event::EventHandler;

pub struct App {
	active: bool,
	data: Data,
	terminal: Terminal<CrosstermBackend<Stdout>>,
	start_time: std::time::Instant,
	tick_count: u32,
	render_count: u32,
	scenes: Vec<Scene>,
}

impl App {
	pub fn new() -> Result<Self> {
		Ok(App {
			active: true,
			data: Default::default(),
			terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
			start_time: std::time::Instant::now(),
			tick_count: 0,
			render_count: 0,
			scenes: vec![],
		})
	}

	pub async fn start(&mut self, tick_rate: u64, frame_rate: u64) -> Result<()> {
		let mut events = EventHandler::new(tick_rate, frame_rate);
		events.start();

		// let base = Scene::new_animation("base scene", None, draw_base);
		let base = Scene::new("Base scene", base_update, base_draw);
		self.scenes.push(base);

		loop {
			let event = events.next().await?;

			self.update(event)?;

			if !self.active {
				break;
			}
		}
		Ok(())
	}

	fn update(&mut self, event: Event) -> Result<()> {
		// global exit
		let exit = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
		if let Event::Key(key) = event {
			if key == exit {
				self.active = false
			}
		}

		match event {
			Event::Error => self.error(),
			// remove completed scenes
			Event::Tick => {
				self.tick_count += 1;
				// TODO: delete done scenes
				// self.scenes.iter_mut().filter(|s| !s.done).collect::<Vec<Scene>>();
				self.scenes
					.iter_mut()
					.map(|s| {
						s.tick_count += 1;
						if let Some(max) = s.max_tick_count {
							if s.tick_count >= max {
								s.done = true;
							}
						}
					})
					.collect()
			}
			Event::Render | Event::Resize(_, _) => self.render()?,
			// on key press / mouse click / paste forward / tick to the current scene
			event => {
				let len = self.scenes.len();
				for i in 0..len - 1 {
					if let Some(update_scene) = self.scenes[len - i].update {
						self.scenes[len - i].done = update_scene(self, event)?;
						break;
					}
				}
			}
		}

		Ok(())
	}

	fn render(&mut self) -> Result<()> {
		self.render_count += 1;

		let len = self.scenes.len();
		let mut depth = 1;
		for i in 0..len - 1 {
			if !self.scenes[len - i].transparent {
				depth = i;
			}
		}

		for i in len - depth..len {
			self.scenes[i].draw_count += 1;
			(self.scenes[i].draw)(self)?;
		}

		Ok(())
	}

	// TODO: this
	fn error(&mut self) {
		let popup =
			Scene::new_transparent(format!("Error {}", self.tick_count), error_update, error_draw);
		self.scenes.push(popup);
	}
}

fn error_update(app: &mut App, event: Event) -> Result<bool> {
	if let Event::Key(key) = event {
		if key == KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)
			|| key == KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)
			|| key == KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)
		{
			return Ok(true);
		}
	}
	Ok(false)
}

fn error_draw(app: &mut App) -> Result<()> {
	// i dont even know if its possible to draw this
	app.terminal.draw(|f| {
		f.render_widget(
			Block::default()
				.title(format!("Error at {}", app.tick_count))
				.borders(Borders::ALL)
				.border_type(BorderType::Rounded)
				.style(Style::new().red()),
			f.size(), // TODO: centre the rect
		);
	})?;
	Ok(())
}

// i cant figure out how to give scenes their own
// specific state so GLOBAL STATE ALL THE TIME IT IS
/// A scene in the app
///
/// the `update` function returns a bool which states whether the scene is done
/// and ready to be cleared on the next tick
///
/// the `draw` function is called on every render cycle when the scene is
/// visable i.e. the scenes above it are transparent
/// 
/// if the scene has no `max_tick_count` and no `update` function to change the
/// `done` state, the scene is permanent and never removed
pub struct Scene {
	name: String,
	done: bool,
	tick_count: u32,
	draw_count: u32,
	transparent: bool,
	max_tick_count: Option<u32>,
	update: Option<fn(&mut App, Event) -> Result<bool>>,
	draw: fn(&mut App) -> Result<()>,
}

impl Scene {
	pub fn new<S: Into<String>>(
		name: S,
		on_update: fn(&mut App, Event) -> Result<bool>,
		on_draw: fn(&mut App) -> Result<()>,
	) -> Self {
		Scene {
			name: name.into(),
			done: false,
			tick_count: 0,
			draw_count: 0,
			transparent: false,
			max_tick_count: None,
			update: Some(on_update),
			draw: on_draw,
		}
	}

	pub fn new_transparent<S: Into<String>>(
		name: S,
		on_update: fn(&mut App, Event) -> Result<bool>,
		on_draw: fn(&mut App) -> Result<()>,
	) -> Self {
		Scene {
			name: name.into(),
			done: false,
			tick_count: 0,
			draw_count: 0,
			transparent: true,
			max_tick_count: None,
			update: Some(on_update),
			draw: on_draw,
		}
	}

	pub fn new_animation<S: Into<String>>(
		name: S,
		max_tick_count: Option<u32>,
		on_draw: fn(&mut App) -> Result<()>,
	) -> Self {
		Scene {
			name: name.into(),
			done: false,
			tick_count: 0,
			draw_count: 0,
			transparent: true,
			max_tick_count: max_tick_count,
			update: None,
			draw: on_draw,
		}
	}
}

impl Default for Scene {
	fn default() -> Scene {
		Scene {
			name: "default name".into(),
			done: false,
			tick_count: 0,
			draw_count: 0,
			transparent: false,
			max_tick_count: None,
			update: None,
			draw: base_draw,
		}
	}
}

fn base_update(app: &mut App, event: Event) -> Result<bool> {
	// TODO: this

	Ok(false)
}

fn base_draw(app: &mut App) -> Result<()> {
	app.terminal.draw(|f| {
		let layout = Layout::new(Direction::Vertical, [Constraint::Min(3), Constraint::Min(1)])
			.split(f.size());
		f.render_widget(
			Paragraph::new(format!(
				"  total ticks: {}\n  total renders: {}",
				app.tick_count, app.render_count
			)),
			layout[0],
		);
		f.render_widget(
			List::new(app.scenes.iter().map(|s| s.name.clone()).collect::<Vec<String>>()).block(
				Block::default()
					.title("open scenes")
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded),
			),
			layout[1],
		);
	})?;

	Ok(())
}
