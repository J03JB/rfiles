use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use rfiles::ActivePanel;
use rfiles::actions::{new_folder, open_file};
use rfiles::draw;
use rfiles::files::{cur_dir, list_files, parent_dir};
use rfiles::preview::preview_me_daddy;
use rfiles::tui::Tui;

use ratatui::widgets::{Clear, ListState};
use std::{io, time::Duration};
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    let mut tui = Tui::new(io::stdout())?;
    tui.enter()?;

    let current_dir = cur_dir(".")?;
    let parent_dir = parent_dir(".");
    // let file_list = list_files(".")?;
    // let mut selected_index = 0;
    let mut active_panel = ActivePanel::Current;
    let mut selected_index_parent = 0;
    let mut selected_index_current = 0;
    let mut parent_list_state = ListState::default();
    let mut current_list_state = ListState::default();
    // let mut list_state = ListState::default();
    // list_state.select(Some(selected_index));
    current_list_state.select(Some(selected_index_current));
    loop {
        // let (_display_name, file_name) = &current_dir[selected_index];
        let (_display_name, file_name) = match active_panel {
            ActivePanel::Parent => &parent_dir[selected_index_parent],
            ActivePanel::Current => &current_dir[selected_index_current],
        };
        let current_path = ".";
        let rt = Runtime::new().unwrap();
        let preview_text = rt.block_on(preview_me_daddy(file_name));
        tui.terminal.draw(|f| {
            f.render_widget(Clear, f.area());
            draw::render_ui(
                f,
                &current_dir,
                &parent_dir,
                selected_index_current,
                selected_index_parent,
                &mut current_list_state,
                &mut parent_list_state,
                &preview_text,
                &active_panel,
                current_path,
            );
        })?;
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break, // Exit on 'q'
                    KeyCode::Down => match active_panel {
                        ActivePanel::Parent => {
                            if selected_index_parent < parent_dir.len().saturating_sub(1) {
                                selected_index_parent += 1;
                            }
                            parent_list_state.select(Some(selected_index_parent));
                        }
                        ActivePanel::Current => {
                            if selected_index_current < current_dir.len().saturating_sub(1) {
                                selected_index_current += 1;
                            }
                            current_list_state.select(Some(selected_index_current));
                        }
                    },
                    KeyCode::Up => match active_panel {
                        ActivePanel::Parent => {
                            if selected_index_parent > 0 {
                                selected_index_parent -= 1;
                            }
                            parent_list_state.select(Some(selected_index_parent));
                        }
                        ActivePanel::Current => {
                            if selected_index_current > 0 {
                                selected_index_current -= 1;
                            }
                            current_list_state.select(Some(selected_index_current));
                        }
                    },
                    KeyCode::Left => {
                        active_panel = ActivePanel::Parent;
                    }

                    KeyCode::Right => {
                        active_panel = ActivePanel::Current;
                    }

                    // KeyCode::Down => {
                    //     if selected_index < file_list.len().saturating_sub(1) {
                    //         selected_index += 1;
                    //     }
                    // }
                    // KeyCode::Up => {
                    //     if selected_index > 0 {
                    //         selected_index = selected_index.saturating_sub(1);
                    //     }
                    // }
                    KeyCode::Enter => open_file(file_name)?,
                    KeyCode::Char('N') => new_folder("Testing")?,
                    _ => {}
                }
                // list_state.select(Some(selected_index));
            }
        }
    }

    tui.exit()?;
    Ok(())
}
