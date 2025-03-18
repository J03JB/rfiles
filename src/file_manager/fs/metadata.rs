// // file_manager/fs/metadata.rs
// use std::path::{Path, PathBuf};
// use crate::file_manager::entry::FileEntry;
//
// pub fn get_directory_contents(path: &Path) -> std::io::Result<Vec<FileEntry>> {
//     // This is the same implementation as read_dir_entries from before
//     let mut entries = Vec::new();
//
//     // Add parent directory entry (..) if not at root
//     if let Some(parent) = path.parent() {
//         // Parent directory logic...
//         entries.push(FileEntry::create_parent_entry(parent));
//     }
//
//     // Read directory entries
//     for entry in std::fs::read_dir(path)? {
//         let entry = entry?;
//         let file_entry = FileEntry::from_path(entry.path())?;
//         entries.push(file_entry);
//     }
//
//     // Sort entries
//     entries.sort();
//
//     Ok(entries)
// }
