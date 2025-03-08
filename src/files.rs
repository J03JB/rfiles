use anyhow::Result;
use devicons::FileIcon;
use std::{fs, path::{Path, PathBuf}};

pub fn list_files(path: &str) -> Result<Vec<String>> {
    let entries = fs::read_dir(PathBuf::from(path))?;
    let mut files = vec![];

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".into());

        let icon = if is_directory(path)    {
            "📁".to_string() // Use a default folder icon
        } else {
                FileIcon::from(&path)
                .to_string()
            };
        files.push(format!("{} {}", icon, file_name));

    }


    Ok(files)
}

pub fn is_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}
