mod event;
mod handler;
mod files;
mod draw;
mod preview;
mod actions;
mod tui;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
