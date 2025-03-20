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
