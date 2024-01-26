use std::process::Command;

use webbrowser;

pub fn open_obsidian(path: String) {
    // Check if Obsidian is installed on the system unix or windows
    let is_windows = cfg!(target_os = "windows");
    let obsidian_installed = Command::new( if is_windows { "which" } else { "whereis" })
        .arg("obsidian")
        .status()
        .expect("Failed to check if Obsidian is installed")
        .success();

    if obsidian_installed {
        let path = std::path::Path::new(&path).join("README.md");
        // convert path to string
        let path = path.to_str().unwrap();
        // normalize path with regex
        let path = path.replace(r"\\", "/");
        // normalize different path separators
        let path = path.replace(r"\", "/");
        // normalize path drive letter
        println!("{}", path);
        // convert unix path to windows path to encoded uri
        let path = percent_encoding::utf8_percent_encode(&path, percent_encoding::NON_ALPHANUMERIC).to_string();
        let obsidian_url = format!("obsidian://open?file={}", path);

        // Open Obsidian using the webbrowser crate
        if let Err(err) = webbrowser::open(&obsidian_url) {
            eprintln!("Failed to open Obsidian: {}", err);
        }
    } else {
        eprintln!("Obsidian is not installed.");
    }
}
