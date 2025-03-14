use anyhow::Result;
use devicons::{FileIcon, icon_for_file};
use std::{
    fs,
    path::{Path, PathBuf},
};

use std::fs::OpenOptions;
use std::io::Write;

fn log_debug(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();
    writeln!(file, "{}", msg).unwrap();
}
pub fn list_files(path: &str) -> Result<Vec<(String, String)>> {
    // let entries = fs::read_dir(PathBuf::from(path))?;
    let mut files = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            // let entry = entry?;
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            // .unwrap_or_else(|_| "Invalid UTF-8".into());

            // let icon = icon_for_file(path,  &None);
            let icon = icons(file_name.clone());

            let display_name = format!("{} {}", icon, &file_name);
            files.push((display_name, file_name.to_string()));
        }
    }

    Ok(files)
}

pub fn cur_dir(path: &str) -> Result<Vec<(String, String)>> {
    list_files(path)
}

pub fn up_dir(path: &str) -> Vec<(String, String)> {
// pub fn up_dir(path: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    // let absolute_path = PathBuf::from(path).canonicalize().expect("erroe");
    // let path_str = absolute_path.to_str().ok_or("erroroa")?;
    // list_files(path_str).map_err(Into::into)
    
    let mut files = Vec::new();

    // Convert path to an absolute path
    let absolute_path = PathBuf::from(path).canonicalize().ok();
    if let Some(parent_path) = absolute_path.as_ref().and_then(|p| p.parent()) {
        if let Ok(entries) = fs::read_dir(parent_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                let icon = icons(file_name.clone()); // Add icon (if needed)
                let display_name = format!("{} {}", icon, &file_name);

                files.push((display_name, file_name));
            }
        }
    }

    files
}

// pub fn up_dir(path: &str) -> Result<Vec<(String, String)>> {
//     let parent = Path::new(path).parent().map(|p| p.to_str().unwrap_or("")).unwrap_or("");
//     list_files(parent)
// }
pub fn icons(file_name: String) -> String {
    FileIcon::from(&file_name).to_string()
}

pub fn is_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}
