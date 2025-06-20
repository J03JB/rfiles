use devicons::FileIcon;
use ratatui::style::Color;
use std::cmp::Ordering;
use std::fs::{self};
use std::path::{Path, PathBuf};

use crate::utils::hex_to_tui_color;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_file: bool,
    // pub size: u64,
    // pub modified: SystemTime,
    // pub permissions: Permissions,
}

#[derive(Debug, Clone, Copy)]
pub struct Permissions {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl FileEntry {
    // Create a new FileEntry from a path
    pub fn from_path(path: PathBuf) -> Result<Self, std::io::Error> {
        // let metadata = fs::metadata(&path)?;
        let filename = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from(".."));

        Ok(Self {
            name: filename,
            is_dir: path.is_dir(),
            is_file: !path.is_dir(),
            // size: if metadata.is_dir() { 0 } else { metadata.len() },
            // modified: metadata.modified().unwrap_or(UNIX_EPOCH),
            // permissions: Permissions::from_metadata(&metadata),
            path,
        })
    }

    // Format the file size for display
    // pub fn format_size(&self) -> String {
    //     if self.is_dir {
    //         return String::from("-");
    //     }
    //
    //     const KB: u64 = 1024;
    //     const MB: u64 = KB * 1024;
    //     const GB: u64 = MB * 1024;
    //
    //     if self.size < KB {
    //         format!("{}B", self.size)
    //     } else if self.size < MB {
    //         format!("{:.1}K", self.size as f64 / KB as f64)
    //     } else if self.size < GB {
    //         format!("{:.1}M", self.size as f64 / MB as f64)
    //     } else {
    //         format!("{:.1}G", self.size as f64 / GB as f64)
    //     }
    // }

    // Format the modification time for display
    // pub fn format_modified(&self) -> String {
    //     use chrono::{DateTime, Local};
    //
    //     let datetime: DateTime<Local> = self.modified.into();
    //     datetime.format("%Y-%m-%d %H:%M").to_string()
    // }

    pub fn extension(&self) -> String {
        if self.is_dir {
            String::new()
        } else {
            self.path
                .extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_default()
        }
    }

    pub fn get_icons(&self) -> (String, Color) {
        let is_directory = self.is_dir || Path::new(&self.path).is_dir();

        if is_directory {
            (FileIcon::from(&self.name).to_string(), Color::Blue)
        } else {
            let icon = FileIcon::from(&self.name);
            let hex_color = icon.color;

            let color = hex_to_tui_color(hex_color);

            (icon.to_string(), color)
        }
    }
}

impl PartialEq for FileEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for FileEntry {}

impl PartialOrd for FileEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_dir, other.is_dir) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
        }
    }
}

// impl Permissions {
//     // pub fn from_metadata(metadata: &Metadata) -> Self {
//         #[cfg(unix)]
//         {
//             use std::os::unix::fs::PermissionsExt;
//             // let mode = metadata.permissions().mode();
//             Self {
//                 readable: mode & 0o444 != 0,
//                 writable: mode & 0o222 != 0,
//                 executable: mode & 0o111 != 0,
//             }
//         }
//
//         #[cfg(windows)]
//         {
//             Self {
//                 readable: true,
//                 writable: !metadata.permissions().readonly(),
//                 executable: false, // Windows doesn't have a direct executable bit
//             }
//         }
//     }
//
//     pub fn to_string(&self) -> String {
//         let r = if self.readable { "r" } else { "-" };
//         let w = if self.writable { "w" } else { "-" };
//         let x = if self.executable { "x" } else { "-" };
//         format!("{}{}{}", r, w, x)
//     }
// }

// Function to read directory contents and convert to FileEntries
pub fn read_dir_entries(path: &Path) -> Result<Vec<FileEntry>, std::io::Error> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_entry = FileEntry::from_path(entry.path())?;
        entries.push(file_entry);
    }

    entries.sort();

    Ok(entries)
}
