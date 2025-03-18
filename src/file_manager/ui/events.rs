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
        if event::poll(self.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                return Ok(InputEvent::Key(key));
            }
        }
        Ok(InputEvent::Tick)
    }
}

// Handle all key events for the file manager
pub fn handle_key_event(file_manager: &mut FileManager, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') => {
            // Quit will be handled in the main loop
            return Ok(());
        }
        KeyCode::Tab => {
            // Cycle through panes
            let next_pane = match file_manager.state.active_pane {
                PaneState::Parent => PaneState::Current,
                PaneState::Current => PaneState::Preview,
                PaneState::Preview => PaneState::Parent,
            };
            file_manager.change_focus(next_pane)?;
        }
        KeyCode::Right | KeyCode::Enter => {
            // Handle navigation into directories
            match file_manager.state.active_pane {
                PaneState::Parent => {
                    file_manager.shift_directories_forward()?;
                }
                PaneState::Current => {
                    // Get selected entry
                    let current_index = *file_manager
                        .state
                        .selected_indices
                        .get(&PaneState::Current)
                        .unwrap_or(&0);

                    // If it's a directory, navigate into it
                    if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
                        if entry.is_dir {
                            file_manager.shift_directories_forward()?;
                        }
                    }
                }
                _ => {}
            }
        }
        KeyCode::Left => {
            // Handle navigation to parent directory
            match file_manager.state.active_pane {
                PaneState::Current => {
                    file_manager.shift_directories_backward()?;
                }
                PaneState::Preview => {
                    file_manager.change_focus(PaneState::Current)?;
                }
                _ => {}
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Move selection up
            let active_pane = file_manager.state.active_pane;
            let index = *file_manager
                .state
                .selected_indices
                .get(&active_pane)
                .unwrap_or(&0);

            if index > 0 {
                file_manager
                    .state
                    .selected_indices
                    .insert(active_pane, index - 1);

                // Always update preview pane when in Current pane and selection changes
                if active_pane == PaneState::Current {
                    file_manager.update_preview_pane();
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Move selection down
            let active_pane = file_manager.state.active_pane;
            let index = *file_manager
                .state
                .selected_indices
                .get(&active_pane)
                .unwrap_or(&0);
            let pane_index = active_pane.to_index();

            if index + 1 < file_manager.panes[pane_index].contents.len() {
                file_manager
                    .state
                    .selected_indices
                    .insert(active_pane, index + 1);

                // Always update preview pane when in Current pane and selection changes
                if active_pane == PaneState::Current {
                    file_manager.update_preview_pane();
                }
            }
        }
        _ => {}
    }

    Ok(())
}
// use std::time::Duration;
// use crossterm::event::{self, Event, KeyCode, KeyEvent};
// use crate::file_manager::{FileManager, PaneState};
//
// pub enum InputEvent {
//     Key(KeyEvent),
//     Tick,
// }
//
// pub struct EventHandler {
//     tick_rate: Duration,
// }
//
// impl EventHandler {
//     pub fn new(tick_rate: Duration) -> Self {
//         Self { tick_rate }
//     }
//
//     pub fn next(&self) -> Result<InputEvent, Box<dyn std::error::Error>> {
//         // Event polling logic...
//         Ok(InputEvent::Tick)
//     }
// }
//
// #[allow(dead_code)]
// // Map crossterm key events to our application events
//  fn handle_key_event(file_manager: &mut FileManager, key: KeyEvent) -> Result<(), &'static str> {
//     match key.code {
//         // KeyCode::Tab => file_manager.change_focus(match file_manager.state.active_pane {
//         //     PaneState::Parent => PaneState::Current,
//         //     PaneState::Current => PaneState::Preview,
//         //     PaneState::Preview => PaneState::Parent,
//         // }),
//         //
//                 KeyCode::Right | KeyCode::Enter => {
//                     // Handle navigation into directories
//                     match file_manager.state.active_pane {
//                         file_manager::PaneState::Parent => {
//                             file_manager
//                                 .shift_directories_forward()
//                                 .map_err(|e| anyhow::anyhow!(e))?;
//                         }
//                         file_manager::PaneState::Current => {
//                             // Get selected entry
//                             let current_index = *file_manager
//                                 .state
//                                 .selected_indices
//                                 .get(&file_manager::PaneState::Current)
//                                 .unwrap_or(&0);
//
//                             // If it's a directory, navigate into it
//                             if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
//                                 if entry.is_dir {
//                                     file_manager
//                                         .shift_directories_forward()
//                                         .map_err(|e| anyhow::anyhow!(e))?;
//                                 }
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//         //
//         // KeyCode::Left => match file_manager.state.active_pane {
//         //     PaneState::Parent => Ok(()),
//         //     PaneState::Current => file_manager.shift_directories_backward(),
//         //     PaneState::Preview => file_manager.change_focus(PaneState::Current),
//         // },
//         //
//         // KeyCode::Up => file_manager.move_selection(-1),
//         // KeyCode::Down => file_manager.move_selection(1),
//
//         _ => Ok(()),
//     }
// }
