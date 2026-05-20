// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn convert_file(name: &str, convert_to: &str, folder: &str) -> Result<String, String> {
    let img = image::open(name).map_err(|e| e.to_string())?;
    let path =  std::path::Path::new(name);
    let mut new_path = path.with_extension(convert_to);

    if folder != ""{
        let new_folder = std::path::Path::new(folder);
        new_path = new_folder.join(new_path.file_name().ok_or("ファイル名を取得できません")?);
    }

    if new_path.exists() {
        let mut new_file_name = std::ffi::OsString::from(new_path.file_stem().map(|m| m.to_owned()).ok_or("ファイル名を取得できません")?);
        new_file_name.push("-copy");
        new_path.set_file_name(new_file_name);

        if let Some(ext) = new_path.extension().map(|e| e.to_owned()){
            new_path.set_extension(ext);
        }
    }

    if convert_to == "gif"{
        let new_file = std::fs::File::create(&new_path).map_err(|e| e.to_string())?;
        let mut encoder = image::codecs::gif::GifEncoder::new(new_file);
        let rgb_img = img.to_rgba8();
        let frame = image::Frame::new(rgb_img);

        encoder.encode_frame(frame).map_err(|e| e.to_string())?;
    }else{
        img.save(&new_path).map_err(|e| e.to_string())?;
    }

    Ok("完了".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![convert_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
