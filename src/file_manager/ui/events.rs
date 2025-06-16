use crate::file_manager::fs::operations;
use crate::file_manager::ui::popup_input::PopupMode;
use crate::file_manager::{FileManager, PaneState};
use crossterm::event::{KeyCode, KeyEvent};
use std::path::{Path, PathBuf};
use anyhow::Result;

fn process_popup_result(file_manager: &mut FileManager, input: String) -> Result<()> {
    match file_manager.input_popup.mode {
        PopupMode::NewFile => {
            if !input.is_empty() {
                let mut path = file_manager.panes[1].path.clone();
                path.push(&input);
                let _ = std::fs::File::create(path);
                file_manager.panes[1].reload_contents()?;
                file_manager.update_preview_pane();
            }
        }
        PopupMode::NewFolder => {
            if !input.is_empty() {
                let mut path = file_manager.panes[1].path.clone();
                path.push(&input);
                let _ = std::fs::create_dir_all(path);
                file_manager.panes[1].reload_contents()?;
                file_manager.update_preview_pane();
            }
        }
        PopupMode::Rename => {
            if !input.is_empty() {
                let current_index = *file_manager
                    .state
                    .selected_indices
                    .get(&PaneState::Current)
                    .unwrap_or(&0);
                if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
                    let old_path = entry.path.clone();
                    let mut new_path = old_path.parent().unwrap_or(Path::new("")).to_path_buf();
                    new_path.push(input);
                    let _ = std::fs::rename(old_path, new_path);
                    file_manager.panes[1].reload_contents()?;
                    file_manager.update_preview_pane();
                }
            }
        }
        PopupMode::Delete => {
            if input.to_lowercase() == "y" {
                let current_index = *file_manager
                    .state
                    .selected_indices
                    .get(&PaneState::Current)
                    .unwrap_or(&0);
                if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
                    let _ = operations::delete_file(&entry.path);
                    file_manager.panes[1].reload_contents()?;
                    file_manager.update_preview_pane();
                }
            }
        }
        PopupMode::Copy => {
            if !input.is_empty() {
                let current_index = *file_manager
                    .state
                    .selected_indices
                    .get(&PaneState::Current)
                    .unwrap_or(&0);
                if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
                    let dest_path = PathBuf::from(input);
                    let _ = operations::copy_file(&entry.path, &dest_path);
                    file_manager.panes[1].reload_contents()?;
                    file_manager.update_preview_pane();
                }
            }
        }
        PopupMode::Move => {
            if !input.is_empty() {
                let current_index = *file_manager
                    .state
                    .selected_indices
                    .get(&PaneState::Current)
                    .unwrap_or(&0);
                if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
                    let dest_path = PathBuf::from(input);
                    let _ = operations::move_file(&entry.path, &dest_path);
                    file_manager.panes[1].reload_contents()?;
                    file_manager.update_preview_pane();
                }
            }
        }
    }
    Ok(())
}

pub fn handle_key_event(file_manager: &mut FileManager, key: KeyEvent) -> Result<(), anyhow::Error> {
    if file_manager.input_popup.active {
        if let Some(input) = file_manager.input_popup.handle_input(key) {
            process_popup_result(file_manager, input)?;
        }
        return Ok(());
    }

    match key.code {
        KeyCode::Enter => {
            match file_manager.state.active_pane {
                PaneState::Parent => {
                    file_manager.shift_directories_forward()?;
                }
                PaneState::Current => {
                    let current_index = *file_manager
                        .state
                        .selected_indices
                        .get(&PaneState::Current)
                        .unwrap_or(&0);

                    if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
                        if entry.is_dir {
                            file_manager.shift_directories_forward()?;
                        } else if entry.is_file {
                            let _ = operations::open_file(&entry.path);
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        }
        KeyCode::Char('N') => {
            file_manager.input_popup.open(PopupMode::NewFolder);
            Ok(())
        }
        KeyCode::Char('n') => {
            file_manager.input_popup.open(PopupMode::NewFile);
            Ok(())
        }
        KeyCode::Char('r') => {
            file_manager.input_popup.open(PopupMode::Rename);
            Ok(())
        }
        KeyCode::Char('d') => {
            file_manager.input_popup.open(PopupMode::Delete);
            Ok(())
        }
        KeyCode::Char('c') => {
            file_manager.input_popup.open(PopupMode::Copy);
            Ok(())
        }
        KeyCode::Char('m') => {
            file_manager.input_popup.open(PopupMode::Move);
            Ok(())
        }
        // KeyCode::Tab => {
        //     let new_focus = match file_manager.state.active_pane {
        //         PaneState::Parent => PaneState::Current,
        //         PaneState::Current => PaneState::Preview,
        //         PaneState::Preview => PaneState::Parent,
        //     };
        //     let _  = file_manager.change_focus(new_focus);
        //     Ok(())
        // }
        KeyCode::Right => {
            match file_manager.state.active_pane {
                PaneState::Parent => {
                    file_manager.shift_directories_forward()?;
                }
                // PaneState::Current => file_manager.shift_directories_forward()?,
                PaneState::Current => {
                    let current_index = *file_manager
                        .state
                        .selected_indices
                        .get(&PaneState::Current)
                        .unwrap_or(&0);

                    if let Some(entry) = file_manager.panes[1].contents.get(current_index) {
                        if entry.is_dir {
                            file_manager.shift_directories_forward()?;
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        }
        KeyCode::Left | KeyCode::Char('h') => match file_manager.state.active_pane {
            PaneState::Current => file_manager.shift_directories_backward(),
            PaneState::Preview => file_manager.change_focus(PaneState::Current),
            _ => Ok(()),
        },
        KeyCode::Up | KeyCode::Char('k') => {
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

                if active_pane == PaneState::Current {
                    file_manager.update_preview_pane();
                }
            }
            Ok(())
        }
        KeyCode::Down | KeyCode::Char('j') => {
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

                if active_pane == PaneState::Current {
                    file_manager.update_preview_pane();
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
