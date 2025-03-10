use crate::event::Event;
use crate::Result;
use crate::files::is_file;
use crate::actions::open_file;

use crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn handle_key_events(
    key_event: KeyEvent,
    // action: &mut Action, 
    sender: UnboundedSender<Event>,
    file_name: &str
) -> Result<()> {
    if is_file(file_name) {
        match key_event.code {
            KeyCode::Enter => {
                open_file(file_name)?
            }
            _ => {}
        }

    }
        return Ok(());
}

