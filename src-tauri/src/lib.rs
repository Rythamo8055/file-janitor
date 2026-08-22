mod scanner;
use scanner::{FileGroup, scan_folders as scan_core};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn scan_folders(paths: Vec<String>) -> Result<Vec<FileGroup>, String> {
    scan_core(paths)
}

#[tauri::command]
fn trash_files(paths: Vec<String>) -> Result<(), String> {
    // trash crate moves to OS trash, not delete
    trash::delete_all(&paths).map_err(|e| e.to_string())
}

#[tauri::command]
fn rename_file(from: String, to: String) -> Result<(), String> {
    if from == to { return Ok(()); }
    std::fs::rename(&from, &to).map_err(|e| format!("rename {}->{}: {}", from, to, e))
}

#[tauri::command]
fn regex_rename(paths: Vec<String>, pattern: String, replacement: String) -> Result<Vec<String>, String> {
    let re = regex::Regex::new(&pattern).map_err(|e| e.to_string())?;
    let mut new_paths = Vec::new();
    for p in paths {
        let path = std::path::Path::new(&p);
        let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let new_name = re.replace_all(fname, replacement.as_str()).to_string();
        if new_name != fname {
            let new_path = path.with_file_name(new_name);
            std::fs::rename(path, &new_path).map_err(|e| e.to_string())?;
            new_paths.push(new_path.to_string_lossy().to_string());
        } else {
            new_paths.push(p);
        }
    }
    Ok(new_paths)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, scan_folders, trash_files, rename_file, regex_rename])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
