pub mod actions;
pub mod draw;
pub mod event;
pub mod files;
pub mod handler;
pub mod preview;
pub mod tui;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// TODO:
// move to somewhere better
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Parent,
    Current,
}

