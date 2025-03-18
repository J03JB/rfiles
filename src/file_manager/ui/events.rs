use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crate::file_manager::{FileManager, PaneState};

pub enum InputEvent {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }
    
    pub fn next(&self) -> Result<InputEvent, Box<dyn std::error::Error>> {
        // Event polling logic...
        Ok(InputEvent::Tick)
    }
}

// Map crossterm key events to our application events
pub fn handle_key_event(file_manager: &mut FileManager, key: KeyEvent) -> Result<(), &'static str> {
    match key.code {
        // KeyCode::Tab => file_manager.change_focus(match file_manager.state.active_pane {
        //     PaneState::Parent => PaneState::Current,
        //     PaneState::Current => PaneState::Preview,
        //     PaneState::Preview => PaneState::Parent,
        // }),
        //
        // KeyCode::Right | KeyCode::Enter => match file_manager.state.active_pane {
        //     PaneState::Parent => file_manager.shift_directories_forward(),
        //     PaneState::Current => {
        //         // Enter directory logic...
        //         Ok(())
        //     },
        //     PaneState::Preview => Ok(()),
        // },
        //
        // KeyCode::Left => match file_manager.state.active_pane {
        //     PaneState::Parent => Ok(()),
        //     PaneState::Current => file_manager.shift_directories_backward(),
        //     PaneState::Preview => file_manager.change_focus(PaneState::Current),
        // },
        //
        // KeyCode::Up => file_manager.move_selection(-1),
        // KeyCode::Down => file_manager.move_selection(1),
        
        _ => Ok(()),
    }
}
