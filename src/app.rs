// app state
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

    pub fn tick(&self) {}
}
