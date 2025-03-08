use ansi_to_tui::IntoText;
use ratatui::text::Text;
use tokio::process::Command;
use std::fs;
use crate::files::is_directory;

pub async fn get_file_preview(filename: &str) -> Text<'static> {
    if is_directory(filename) {
        match fs::read_dir(filename) {
            Ok(entries) => {
                let mut result = String::from("📁 Directory contents:\n");
                for entry in entries.flatten() {
                    let name = entry
                        .file_name()
                        .into_string()
                        .unwrap_or_else(|_| "Invalid UTF-8".into());
                    result.push_str(&format!("- {}\n", name));
                }
                result.into()
            }
            Err(_) => "Failed to read directory contents".to_string().into()
        }
    } else {
        let output = Command::new("bat")
            .arg("--style=plain")
            .arg("--paging=never")
            .arg("--color=always")
            .arg(filename)
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
                raw_output.into_text().unwrap_or_else(|_| Text::raw("Failed to parse ANSI"))
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let status_code = output.status;
                format!(
                    "Error: Failed to preview file\nExit Code: {:?}\nStderr: {}",
                    status_code, stderr
                ).into()
            }
            _ => Text::raw("Failed to execute bat: {}"),
        }
    }
}
