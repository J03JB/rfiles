use crate::file_manager::entry::{FileEntry, read_dir_entries};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::SystemTime;
use tokio::process::Command;
use ratatui::text::Text;
use ansi_to_tui::IntoText;

fn log_debug(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();
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

    pub fn reload_contents(&mut self) -> Result<(), &'static str> {
        log_debug(&format!("Reading directory: {:?}", self.path));
        match read_dir_entries(&self.path) {
            Ok(entries) => {
                self.contents = entries;
                Ok(())
            }
            Err(_) => Err("Failed to read directory contents"),
        }
    }

    pub async fn reload_contents_or_preview(&mut self) -> Result<(), &'static str> {
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
        // For files, load preview content asynchronously
        // match Self::preview_file(&self.path.to_string_lossy()).await {
        //     Ok(content) => {
        //         // Store the preview content
        //         self.preview_content = Some(content);
        //         self.contents.clear();
        //         log_debug(&format!("Preview loaded successfully for: {:?}", self.path));
        //         Ok(())
        //     }
        //     Err(err) => {
        //         // Log the error
        //         log_debug(&format!("Preview failed for {:?}: {}", self.path, err));
        //
        //         // Store a simple error message as the preview
        //         self.preview_content = Some(format!("Failed to preview file: {:?}\nError: {}", self.path, err));
        //         self.contents.clear();
        //         Err("Failed to preview file")
        //     }
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
            output
                .stdout
                .into_text()
                .unwrap_or_else(|_| Text::raw("Failed to parse ANSI")) // let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
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

    // pub async fn preview_file(filename: &str) -> Result<String, String> {
    //     let output = Command::new("bat")
    //         .arg("--style=plain")
    //         .arg("--paging=never")
    //         .arg("--color=always")
    //         .arg(filename)
    //         .stdout(Stdio::piped())
    //         .output()
    //         .await
    //         .map_err(|_| "Failed to execute bat".to_string())?;
    //
    //     match output {
    //         Ok(output) if output.status.success() => {
    //             output
    //                 .stdout
    //                 .into_text()
    //                 .unwrap_or_else(|_| Text::raw("Failed to parse ANSI")) // let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
    //         }
    //         Ok(output) => {
    //             let stderr = String::from_utf8_lossy(&output.stderr);
    //             let status_code = output.status;
    //             format!(
    //                 "Error: Failed to preview file\nExit Code: {:?}\nStderr: {}",
    //                 status_code, stderr
    //             )
    //                 .into()
    //         }
    //         _ => Text::raw("Failed to execute bat"),
    //     }

        // if output.status.success() {
        //     String::from_utf8(output.stdout).map_err(|_| "Failed to parse ANSI output".to_string())
        // } else {
        //     let stderr = String::from_utf8_lossy(&output.stderr);
        //     Err(format!(
        //         "Error: Failed to preview file\nExit Code: {:?}\nStderr: {}",
        //         output.status, stderr
        //     ))
        // }
    // }
    pub fn ensure_selection_visible(&mut self, selected_index: usize) {
        if selected_index < self.scroll_offset {
            self.scroll_offset = selected_index;
        } else if selected_index >= self.scroll_offset + self.visible_height {
            self.scroll_offset = selected_index - self.visible_height + 1;
        }
    }
}

    // pub async fn reload_contents_or_preview(&mut self) -> Result<(), &'static str> {
    //     if self.path.is_dir() {
    //         self.reload_contents()
    //     } else {
    //         // For files, load preview content asynchronously
    //         match Self::preview_file(&self.path.to_string_lossy()).await {
    //             Ok(content) => {
    //                 // Do something with the content (e.g., store it in self)
    //                 self.preview_content = Some(content);
    //                 self.contents.clear();
    //                 Ok(())
    //             }
    //             Err(_) => Err("Failed to preview file"),
    //         }
    //     }
    // }

