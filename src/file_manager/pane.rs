use crate::file_manager::entry::{FileEntry, read_dir_entries};
use ansi_to_tui::IntoText;
use anyhow::{Context, Result};
use ratatui::text::Text;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::SystemTime;
use tokio::process::Command;

fn log_debug(msg: &str) {
    let mut file = OpenOptions::new().create(true).append(true).open("debug.log").unwrap();
    writeln!(file, "{}", msg).unwrap();
}
pub struct Pane {
    pub path: PathBuf,
    pub contents: Vec<FileEntry>,
    pub preview_content: Option<Text<'static>>,
    pub scroll_offset: usize,
    pub visible_height: usize,
}

impl Pane {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            contents: Vec::new(),
            preview_content: None,
            scroll_offset: 0,
            visible_height: 0,
        }
    }

    pub fn reload_contents(&mut self) -> Result<()> {
        log_debug(&format!("Reading directory: {:?}", self.path));
        self.contents = read_dir_entries(&self.path)
            .context(format!("Failed to read directory contents for path: {:?}", self.path))?;

        Ok(())
    }

    pub async fn reload_contents_or_preview(&mut self) -> Result<()> {
        // Log the operation for debugging
        log_debug(&format!("Attempting to preview or load: {:?}", self.path));

        if self.path.is_dir() {
            // For directories, load their contents
            self.reload_contents()
        } else {
            // Clear any previous preview content
            self.preview_content = None;

            let preview_content = Self::preview_file(&self.path.to_string_lossy()).await;

            self.preview_content = Some(preview_content);
            self.contents.clear();
            log_debug(&format!("preview loaded successfully for: {:?}", self.path));
            Ok(())
        }
    }

    pub async fn preview_file(filename: &str) -> Text<'static> {
        let output = Command::new("bat")
            .arg("--style=plain")
            .arg("--paging=never")
            .arg("--color=always")
            .arg(filename)
            .stdout(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                output.stdout.into_text().unwrap_or_else(|_| Text::raw("Failed to parse ANSI")) // let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let status_code = output.status;
                format!(
                    "Error: Failed to preview file\nExit Code: {:?}\nStderr: {}",
                    status_code, stderr
                )
                .into()
            }
            _ => Text::raw("Failed to execute bat"),
        }
    }

    pub fn ensure_selection_visible(&mut self, selected_index: usize) {
        if selected_index < self.scroll_offset {
            self.scroll_offset = selected_index;
        } else if selected_index >= self.scroll_offset + self.visible_height {
            self.scroll_offset = selected_index - self.visible_height + 1;
        }
    }
}
