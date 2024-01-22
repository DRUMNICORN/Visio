// obsidian.rs
use std::process::Command;
use webbrowser;

pub fn open_obsidian(path: String) {
    // Check if Obsidian is installed
    let obsidian_installed = Command::new("which")
        .arg("obsidian")
        .status()
        .expect("Failed to check if Obsidian is installed")
        .success();

    if obsidian_installed {
        let path = std::path::Path::new(&path);
        let path = path.to_str().unwrap();
        let path = path.replace("\\", "/");
        let obsidian_url = format!("obsidian://vault={}", path);

        // Open Obsidian using the webbrowser crate
        if let Err(err) = webbrowser::open(&obsidian_url) {
            eprintln!("Failed to open Obsidian: {}", err);
        }
    } else {
        eprintln!("Obsidian is not installed.");
    }
}
