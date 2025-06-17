use anyhow::Result;
use std::io::{stderr, LineWriter, Write};
use std::ops::Deref;
use crossterm::{
    cursor,
    event::DisableMouseCapture,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, is_raw_mode_enabled,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, layout::Size};
use cli_log::*;

pub struct Tui<W>
    where 
        W:Write,
    {
        pub terminal: ratatui::Terminal<CrosstermBackend<W>>,
    }
impl<W> Tui<W>
    where 
    W: Write,
{
    pub fn new(writer: W) -> Result<Self> {
        Ok(Self {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(writer))?,
        })
    }

    pub fn _size(&self) -> Result<Size> {
        Ok(self.terminal.size()?)
    }

    pub fn enter(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut buff_stderr = LineWriter::new(stderr());
        execute!(buff_stderr, EnterAlternateScreen)?;
        self.terminal.clear()?;
        execute!(buff_stderr, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        if is_raw_mode_enabled()? {
            debug!("Exiting terminal");

            disable_raw_mode()?;
            let mut buff_stderr  = LineWriter::new(stderr());
            execute!(buff_stderr, cursor::Show)?;
            execute!(buff_stderr, LeaveAlternateScreen)?;
        }

        Ok(())
    }
}

impl<W> Deref for Tui<W>
where
    W: Write,
{
    type Target = ratatui::Terminal<CrosstermBackend<W>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl<W> Drop for Tui<W>
where
    W: Write,
{
    fn drop(&mut self) {
        match self.exit() {
            Ok(()) => debug!("Successfully exited terminal"),
            Err(e) => debug!("Failed to exit terminal: {:?}", e),
        }
    }
}
