use std::path::Path;
use std::fs::File;
use std::ffi::OsString;

fn convert_by_img2svg_crate<P1: AsRef<Path>, P2: AsRef<Path>>(input: P1, output: P2) -> Result<(), String>{
    let options = img2svg::ConversionOptions{
        ..Default::default()
    };
    img2svg::convert(input.as_ref(), output.as_ref(), &options).map_err(|e| e.to_string())
}
fn convert_by_image_crate<P1: AsRef<Path>, P2: AsRef<Path>>(input: P1,output: P2) -> Result<(), String>{
    let img = image::open(input).map_err(|e| e.to_string())?;
    let ext = output.as_ref().extension().ok_or("拡張子の取得に失敗しました")?.to_string_lossy();
    
    if ext == "gif" {
        let new_file = File::create(output).map_err(|e| e.to_string())?;

        let mut encoder = image::codecs::gif::GifEncoder::new(new_file);
        let rgb_img = img.to_rgba8();
        let frame = image::Frame::new(rgb_img);

        encoder.encode_frame(frame).map_err(|e| e.to_string())?;
    }else{
        img.save(output).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn convert_file(input: &str, convert_to: &str, folder: &str) -> Result<(), String> {
    let path = Path::new(input);
    let mut new_path = path.with_extension(convert_to);
    
    // 保存先が指定されている場合
    if !folder.is_empty() {
        let new_folder = Path::new(folder);
        new_path = new_folder.join(new_path.file_name().ok_or("ファイル名を取得できません")?);
    }

    // ファイル名が重複している場合
    if new_path.exists() {
        let mut new_file_name = OsString::from(
            new_path.file_stem().ok_or("ファイル名の取得に失敗しました")?
        );
        let original_ext = new_path.extension().map(|e| e.to_owned());
        new_file_name.push("-copy");
        new_path.set_file_name(new_file_name);

        if let Some(ext) = original_ext {
            new_path.set_extension(ext);
        }
    }

    // 画像保存
    match convert_to {
        "svg" => {
            convert_by_img2svg_crate(input, &new_path)
        },
        _ => {
            convert_by_image_crate(input, &new_path)
        }
    }
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

#[tauri::command]
fn can_read(input: &str) -> bool {
    image::ImageFormat::from_path(input)
        .map(|f| f.can_read())
        .unwrap_or(false)
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
        .invoke_handler(tauri::generate_handler![convert_file, fetch_args, can_read])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
