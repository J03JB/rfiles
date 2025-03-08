use crate::files::is_directory;
use std::{fs, process::Command};

pub fn get_file_preview(filename: &str) -> String {
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
                result
            }
            Err(_) => "Failed to read directory contents".to_string(),
        }
    } else {
        let output = Command::new("bat")
            .arg("--style=numbers,snip")
            .arg("--color=always")
            .arg("--wrap=never")
            .arg(filename)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).to_string()
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let status_code = output.status;
                format!(
                    "Error: Failed to preview file\nExit Code: {:?}\nStderr: {}",
                    status_code, stderr
                )
            }
            Err(e) => format!("Failed to execute bat: {}", e),
        }
    }
}
