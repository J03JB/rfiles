use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{self, Metadata};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
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
        let filename = path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from(".."));
            
        Ok(Self {
            name: filename,
            is_dir: path.is_dir(),
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
    
    // Get file extension (returns empty string for directories)
    pub fn extension(&self) -> String {
        if self.is_dir {
            String::new()
        } else {
            self.path.extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_default()
        }
    }

    pub fn icons() {
        let is_directory =
        self.is_dir || Path::new(&self.path).is_dir();

        let icon = if is_directory {
            icon_for_file(&entry.path, &Some(Theme::Dark)).to_string()
        } else {
            icon_for_file(&entry.name, &Some(Theme::Dark)).to_string()
        };
    }

    // Check if this is a special directory entry (. or ..)
    pub fn is_special_dir(&self) -> bool {
        self.is_dir && (self.name == "." || self.name == "..")
    }
    
    // Get a character icon based on file type
    pub fn get_icon(&self) -> &'static str {
        if self.is_dir {
            "📁"
        } else {
            match self.extension().as_str() {
                "rs" => "🦀",
                "txt" | "md" => "📄",
                "jpg" | "png" | "gif" => "🖼️",
                "mp3" | "wav" | "flac" => "🎵",
                "mp4" | "avi" | "mkv" => "🎬",
                "pdf" => "📑",
                "zip" | "tar" | "gz" => "📦",
                "exe" | "app" => "⚙️",
                _ => "📎",
            }
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
        // Directories come before files
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
