use ratatui::Frame;
use color_eyre::eyre::Result;

use crate::{app::App, event::Event};

pub struct Scene {
    pub title: String,
    pub tick_count: u32,
    pub draw_count: u32,
    pub draw: fn(&App, &mut Frame) -> Result<()>,
    pub update: fn(&App, Event) -> Result<bool>, // return false when done
    pub animations: Vec<Animation>,
}

impl Scene {
    pub fn incr_tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn incr_draw(&mut self) {
        self.draw_count += 1;
    }
}

pub struct Animation {
    pub name: String,
    pub draw: fn(&App, u32) -> Result<bool>, // return false when done
}
