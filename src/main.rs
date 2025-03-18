mod file_manager;
mod utils;
use file_manager::ui::events::{InputEvent, EventHandler};
use file_manager::FileManager;
use file_manager::tui::Tui;
use file_manager::ui::render;
use std::io;
// use file_manager::ui::events::handle_key_event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    let mut tui = Tui::new(io::stdout())?;
    tui.enter()?;

    // Create file manager
    let mut file_manager = FileManager::new();
    // file_manager.init_paths().map_err(|e| anyhow::anyhow!(e))?;
    file_manager.init_paths()?;

    // Run the application
    let event_handler = EventHandler::new(std::time::Duration::from_millis(100));
    let result = run_app(&mut tui, &mut file_manager,  &event_handler);

    tui.exit()?;

    result
}

fn run_app(
    tui: &mut Tui<io::Stdout>,
    file_manager: &mut FileManager,
    event_handler: &EventHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    use file_manager::ui::events::handle_key_event;

    loop {
        tui.terminal.draw(|frame| {
            render::render(file_manager, frame);
        })?;

                // Handle events
        match event_handler.next()? {
            InputEvent::Key(key) => {
                // Check for quit key first
                if let crossterm::event::KeyCode::Char('q') = key.code {
                    break;
                }
                
                // Handle all other keys
                handle_key_event(file_manager, key)?;
            }
            InputEvent::Tick => {
                // Handle tick events if needed (for animations, etc.)
            }
        }
    }

    Ok(())
}

// fn render_ui(frame: &mut tui::Frame, file_manager: &FileManager) {
//     use ratatui::{
//         layout::{Constraint, Direction, Layout},
//         style::{Color, Style},
//         widgets::{Block, Borders, List, ListItem},
//     };
//
//     // Create layout with 3 equal columns
//     let chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage(33),
//             Constraint::Percentage(34),
//             Constraint::Percentage(33),
//         ])
//         .split(frame.size());
//
//     // Render each pane
//     for (i, pane) in file_manager.panes.iter().enumerate() {
//         // Determine if this pane is active
//         let is_active = i == file_manager.state.active_pane.to_index();
//
//         // Get selected index for this pane
//         let selected_index = *file_manager.state.selected_indices
//             .get(&file_manager::PaneState::from_index(i))
//             .unwrap_or(&0);
//
//         // Create list items from directory contents
//         let items: Vec<ListItem> = pane.contents.iter().map(|entry| {
//             let prefix = if entry.is_dir { "📁 " } else { "📄 " };
//             ListItem::new(format!("{}{}", prefix, entry.name))
//         }).collect();
//
//         // Create border style based on active state
//         let border_style = if is_active {
//             Style::default().fg(Color::Yellow)
//         } else {
//             Style::default().fg(Color::White)
//         };
//
//         // Create list widget
//         let list = List::new(items)
//             .block(Block::default()
//                 .title(pane.path.to_string_lossy().to_string())
//                 .borders(Borders::ALL)
//                 .border_style(border_style))
//             .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
//             .highlight_symbol("> ");
//
//         // Render list with selected item
//         let mut state = ratatui::widgets::ListState::default();
//         state.select(Some(selected_index));
//
//         frame.render_stateful_widget(list, chunks[i], &mut state);
//     }
// }
//
// use anyhow::Result;
// use crossterm::event::{self, Event, KeyCode, KeyEvent};
// use rfiles::ActivePanel;
// use rfiles::actions::{new_folder, open_file};
// use rfiles::draw;
// use rfiles::files::{cur_dir, list_files, parent_dir};
// use rfiles::preview::preview_me_daddy;
// use rfiles::tui::Tui;
//
// use ratatui::widgets::{Clear, ListState};
// use std::{io, time::Duration};
// use tokio::runtime::Runtime;
//
// fn main() -> Result<()> {
//     let mut tui = Tui::new(io::stdout())?;
//     tui.enter()?;
//
//     let current_dir = cur_dir(".")?;
//     let parent_dir = parent_dir(".");
//     // let file_list = list_files(".")?;
//     // let mut selected_index = 0;
//     let mut active_panel = ActivePanel::Current;
//     let mut selected_index_parent = 0;
//     let mut selected_index_current = 0;
//     let mut parent_list_state = ListState::default();
//     let mut current_list_state = ListState::default();
//     // let mut list_state = ListState::default();
//     // list_state.select(Some(selected_index));
//     current_list_state.select(Some(selected_index_current));
//     loop {
//         // let (_display_name, file_name) = &current_dir[selected_index];
//         let (_display_name, file_name) = match active_panel {
//             ActivePanel::Parent => &parent_dir[selected_index_parent],
//             ActivePanel::Current => &current_dir[selected_index_current],
//         };
//         let current_path = ".";
//         let rt = Runtime::new().unwrap();
//         let preview_text = rt.block_on(preview_me_daddy(file_name));
//         tui.terminal.draw(|f| {
//             f.render_widget(Clear, f.area());
//             draw::render_ui(
//                 f,
//                 &current_dir,
//                 &parent_dir,
//                 selected_index_current,
//                 selected_index_parent,
//                 &mut current_list_state,
//                 &mut parent_list_state,
//                 &preview_text,
//                 &active_panel,
//                 current_path,
//             );
//         })?;
//         if event::poll(Duration::from_millis(100))? {
//             if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
//                 match code {
//                     KeyCode::Char('q') => break, // Exit on 'q'
//                     KeyCode::Down => match active_panel {
//                         ActivePanel::Parent => {
//                             if selected_index_parent < parent_dir.len().saturating_sub(1) {
//                                 selected_index_parent += 1;
//                             }
//                             parent_list_state.select(Some(selected_index_parent));
//                         }
//                         ActivePanel::Current => {
//                             if selected_index_current < current_dir.len().saturating_sub(1) {
//                                 selected_index_current += 1;
//                             }
//                             current_list_state.select(Some(selected_index_current));
//                         }
//                     },
//                     KeyCode::Up => match active_panel {
//                         ActivePanel::Parent => {
//                             if selected_index_parent > 0 {
//                                 selected_index_parent -= 1;
//                             }
//                             parent_list_state.select(Some(selected_index_parent));
//                         }
//                         ActivePanel::Current => {
//                             if selected_index_current > 0 {
//                                 selected_index_current -= 1;
//                             }
//                             current_list_state.select(Some(selected_index_current));
//                         }
//                     },
//                     KeyCode::Left => {
//                         active_panel = ActivePanel::Parent;
//                     }
//
//                     KeyCode::Right => {
//                         active_panel = ActivePanel::Current;
//                     }
//
//                     // KeyCode::Down => {
//                     //     if selected_index < file_list.len().saturating_sub(1) {
//                     //         selected_index += 1;
//                     //     }
//                     // }
//                     // KeyCode::Up => {
//                     //     if selected_index > 0 {
//                     //         selected_index = selected_index.saturating_sub(1);
//                     //     }
//                     // }
//                     KeyCode::Enter => open_file(file_name)?,
//                     KeyCode::Char('N') => new_folder("Testing")?,
//                     _ => {}
//                 }
//                 // list_state.select(Some(selected_index));
//             }
//         }
//     }
//
//     tui.exit()?;
//     Ok(())
// }
