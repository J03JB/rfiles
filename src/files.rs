use anyhow::Result;
use devicons::{FileIcon, icon_for_file};
use std::{
    fs,
    path::{Path, PathBuf},
};

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

pub fn icons(file_name: String) -> String {
    FileIcon::from(&file_name).to_string()
}

pub fn is_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}
