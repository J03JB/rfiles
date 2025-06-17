mod file_manager;
mod utils;
use anyhow::Result;
use cli_log::*;
use file_manager::FileManager;
use file_manager::tui::Tui;
use file_manager::ui::render;
use std::io;

fn main() -> Result<()> {
    init_cli_log!();
    let mut tui = Tui::new(io::stdout())?;
    tui.enter()?;

    let mut file_manager = FileManager::new();
    file_manager.init_paths()?;

    let result = run_app(&mut tui, &mut file_manager);

    tui.exit()?;

    result
}

fn run_app(
    tui: &mut Tui<io::Stdout>,
    file_manager: &mut FileManager,
    // event_handler: &EventHandler,
) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode};
    use file_manager::ui::events::handle_key_event;

    loop {
        tui.terminal.draw(|frame| {
            render::render(file_manager, frame);
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') if !file_manager.input_popup.active => {
                    break;
                }
                _ => {
                    handle_key_event(file_manager, key).map_err(|e| anyhow::anyhow!(e))?;
                }
            }
        }
    }

    Ok(())
}
