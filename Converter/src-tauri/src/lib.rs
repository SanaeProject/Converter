// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn convert_file(name: &str, convert_to: &str, folder: &str) -> Result<String, String> {
    let img = image::open(name).map_err(|e| e.to_string())?;
    let path = std::path::Path::new(name);
    let mut new_path = path.with_extension(convert_to);

    if !folder.is_empty() {
        let new_folder = std::path::Path::new(folder);
        new_path = new_folder.join(new_path.file_name().ok_or("ファイル名を取得できません")?);
    }

    if new_path.exists() {
        let mut new_file_name = std::ffi::OsString::from(
            new_path.file_stem().ok_or("ファイル名の取得に失敗しました")?
        );
        let original_ext = new_path.extension().map(|e| e.to_owned());
        new_file_name.push("-copy");
        new_path.set_file_name(new_file_name);

        if let Some(ext) = original_ext {
            new_path.set_extension(ext);
        }
    }

    if convert_to == "gif" {
        let new_file = std::fs::File::create(&new_path).map_err(|e| e.to_string())?;
        let mut encoder = image::codecs::gif::GifEncoder::new(new_file);
        let rgb_img = img.to_rgba8();
        let frame = image::Frame::new(rgb_img);

        encoder.encode_frame(frame).map_err(|e| e.to_string())?;
    } else {
        img.save(&new_path).map_err(|e| e.to_string())?;
    }

    Ok("完了".to_string())
}
#[tauri::command]
fn fetch_args() -> Vec<String>{
    let empty = Vec::new();
    ARGS.get()
        .unwrap_or(&empty)
        .iter()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .collect()
}

static ARGS: std::sync::OnceLock<Vec<std::ffi::OsString>> = std::sync::OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(
            |_app| {
                let args: Vec<std::ffi::OsString> = std::env::args_os().collect();
                let file_paths: Vec<std::ffi::OsString> = args.into_iter().skip(1).collect();
                
                let _ = ARGS.set(file_paths);
                Ok(())
            }
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![convert_file, fetch_args])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
