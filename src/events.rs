use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};

#[derive(Debug)]
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

/// Poll the terminal for an input event, returning Tick if nothing arrives
/// within `timeout`. Mouse/resize events are swallowed (not needed for v1).
pub fn poll(timeout: Duration) -> Result<AppEvent> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(k) if k.kind == crossterm::event::KeyEventKind::Press => {
                return Ok(AppEvent::Key(k));
            }
            _ => return Ok(AppEvent::Tick),
        }
    }
    Ok(AppEvent::Tick)
}
