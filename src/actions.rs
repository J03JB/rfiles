use std::process::Command;
use std::io::Result;
// use crate::Result;

pub fn open_file(file_path: &str) -> Result<()> {
    if cfg!(target_os = "macos") {
        // Use the `open` command on macOS
        Command::new("nvim")
            .arg(file_path)
            .spawn()?;
    } else if cfg!(target_os = "linux") {
        // Use the `xdg-open` command on Linux
        Command::new("xdg-open")
            .arg(file_path)
            .spawn()?;
    } else {
        // Handle other operating systems or return an error
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported operating system",
        ));
    }

    Ok(())
}

