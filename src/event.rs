use std::sync::mpsc;

use crossterm::event::{KeyEvent, MouseEvent};

pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: std::thread::JoinHandle<()>
}