// utils.rs
use std::{ffi::OsStr, path::Path};
use ignore::WalkBuilder;

pub fn open_folder_dialog(app: tauri::AppHandle) {
    tauri::api::dialog::FileDialogBuilder::new()
        .pick_folder(|folder| {
            let path = serde_json::to_string(&folder).unwrap();
            tauri::async_runtime::spawn(async move {
                // convert windows path to unix path
                let path = path.replace("//", "\\");
                tauri::Manager::emit_all(&app, "folder-selected", Some(path)).unwrap();
            });
        });
}

pub fn load_folder(path: String) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    // log path to console
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap().to_string();
        let path = path.replace("\\", "/");
        items.push(path);
    }
    items
}

pub fn get_item_count(path: String) -> i32 {
    match std::fs::read_dir(path) {
        Ok(paths) => paths.count().try_into().unwrap(),
        Err(_) => -1,
    }
}

pub fn get_total_count(path: String, filter: Vec<String>) -> i32 {
    let mut total_count = 0;

    // Ensure the path points to a directory
    let path = Path::new(&path);
    if !path.is_dir() {
        return -1;
    }

    let mut excludes = ignore::overrides::OverrideBuilder::new(&path);
    for line in filter {
        excludes.add(&line).unwrap();
    }

    let walker = WalkBuilder::new(path)
        .hidden(false)
        .follow_links(false)
        .git_ignore(false)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = Path::new(entry.path().to_str().unwrap());
                if path.is_file() {
                    let extension = path.extension().and_then(OsStr::to_str);
                    if extension == Some("md") {
                        total_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error encountered while walking the path: {:?}", e);
            }
        }
    }

    total_count
}

pub fn load_gitignore(path: String) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    // read the file
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    // split the file into lines
    let lines = contents.split("\n");
    // iterate over the lines
    for line in lines {
        // if the line is not empty and not a comment
        if line != "" && !line.starts_with("#") {
            // add the line to the items
            items.push(line.to_string());
        }
    }
    items
}
