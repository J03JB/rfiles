use crate::files::{self, is_directory};
use ansi_to_tui::IntoText;
use ratatui::text::Text;
use std::fs;
use tokio::process::Command;

pub fn preview_dir(filename: &str) -> String {
    if let Ok(files) = files::list_files(filename) {


    files
        .iter()
        .map(|(display_name, _file_name)| display_name.clone())
        .collect::<Vec<_>>()
        .join("\n")
    } else  { 
        return "ErrorRRR".to_string()
    } 
}
pub async fn preview_file(filename: &str) -> Text<'static> {
    let output = Command::new("bat")
        .arg("--style=plain")
        .arg("--paging=never")
        .arg("--color=always")
        .arg(filename)
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            // let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
            // raw_output.into_text().unwrap_or_else(|_| Text::raw("Failed to parse ANSI"))
            let raw_output = String::from_utf8_lossy(&output.stdout); // Raw output with ANSI codes
                        raw_output
                .as_ref() // Convert Cow<str> into &str
                .into_text()
                .unwrap_or_else(|_| Text::raw("Failed to parse ANSI"))
            // raw_output.into_text().unwrap_or_else(|_| Text::raw("Failed to parse ANSI"))
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
        _ => Text::raw("Failed to execute bat: {}"),
    }
}

pub async fn preview_me_daddy(filename: &str) -> String {
    if is_directory(filename) {
        preview_dir(filename)
    } else {
        preview_file(filename).await.to_string()
    }
}
