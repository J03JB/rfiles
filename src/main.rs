mod draw;
mod files;
mod preview;
mod tui;

use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use files::list_files;
use preview::get_file_preview;
use ratatui::widgets::{Clear, ListState};
use tokio::runtime::Runtime;
use std::{io, time::Duration};
use tui::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::new(io::stdout())?;
    tui.enter()?; // Initialize TUI

    let file_list = list_files(".")?;
    let mut selected_index = 0;
    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));

    loop {
        let (_display_name, file_name) = &file_list[selected_index];
        let rt = Runtime::new().unwrap();
        let preview_text = rt.block_on(get_file_preview(file_name));
        tui.terminal.draw(|f| {
            f.render_widget(Clear, f.area());
            draw::render_ui(
                f,
                &file_list,
                selected_index,
                &mut list_state,
                &preview_text,
            );
        })?;
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break, // Exit on 'q'
                    KeyCode::Down => {
                        if selected_index < file_list.len().saturating_sub(1) {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index = selected_index.saturating_sub(1);
                        }
                    }
                    _ => {}
                }
                list_state.select(Some(selected_index));
            }
        }
    }

    tui.exit()?;
    Ok(())
}
