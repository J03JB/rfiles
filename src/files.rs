use anyhow::Result;
use std::{fs, path::{Path, PathBuf}};

pub fn list_files(path: &str) -> Result<Vec<String>> {
    let entries = fs::read_dir(PathBuf::from(path))?;
    let mut files = vec![];

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".into());
        files.push(file_name);
    }

    Ok(files)
}

pub fn is_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}
