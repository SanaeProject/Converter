use std::path::Path;
use std::fs::File;
use std::ffi::OsString;

/**
 * img2svgクレートを使用して画像を変換する
 * @args input: 変換元のファイルパス
 * @args output: 変換後のファイルパス
 * @returns 成功した場合はOk、失敗した場合はErrにエラーメッセージを格納して返す
 */
fn convert_by_img2svg_crate<P1: AsRef<Path>, P2: AsRef<Path>>(input: P1, output: P2) -> Result<(), String>{
    let img = img2svg::load_image(input.as_ref()).map_err(|e| e.to_string())?;
    let enhanced_options = img2svg::EnhancedOptions{
        corner_threshold: 45.0,
        num_colors: 12,
        ..Default::default()
    };

    let svg_layers = img2svg::vectorize_enhanced(&img, &enhanced_options).map_err(|e| e.to_string())?;
    img2svg::write_enhanced_svg(&svg_layers, output.as_ref()).map_err(|e| e.to_string())?;

    Ok(())
}

/**
 * imageクレートを使用して画像を変換する
 * @args input: 変換元のファイルパス
 * @args output: 変換後のファイルパス
 * @returns 成功した場合はOk、失敗した場合はErrにエラーメッセージを格納して返す
 */
fn convert_by_image_crate<P1: AsRef<Path>, P2: AsRef<Path>>(input: P1,output: P2) -> Result<(), String>{
    let img = image::open(input.as_ref()).map_err(|e| e.to_string())?;
    let ext = output.as_ref().extension().ok_or("拡張子の取得に失敗しました")?.to_string_lossy();
    
    if ext == "gif" {
        let new_file = File::create(output.as_ref()).map_err(|e| e.to_string())?;

        let mut encoder = image::codecs::gif::GifEncoder::new(new_file);
        let rgb_img = img.to_rgba8();
        let frame = image::Frame::new(rgb_img);

        encoder.encode_frame(frame).map_err(|e| {
            std::fs::remove_file(output.as_ref()).ok();
            e.to_string()
        })?;
    }else{
        img.save(output.as_ref()).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/**
 * 変換可能な拡張子かどうかを確認する
 * @args input: 変換元のファイルパス
 * @returns 変換可能な拡張子の場合はtrue、そうでない場合はfalse
 */
fn can_read<P: AsRef<Path>>(input: P) -> bool {
    fetch_can_read_exts().iter().any(|ext| {
        input.as_ref()
            .extension()
            .map(|f| f.eq_ignore_ascii_case(ext))
            .unwrap_or(false)
    })
}

/**
 * 変換可能な拡張子を取得する
 * @args output: 変換後のファイルパス
 * @returns 変換可能な拡張子のベクター
 */
fn can_convert<P: AsRef<Path>>(output: P) -> bool {
    fetch_can_write_exts().iter().any(|ext|
        output.as_ref()
            .extension()
            .map(|f| f.eq_ignore_ascii_case(ext))
            .unwrap_or(false)    
    )
}

/**
 * 読み込み可能な拡張子を取得する
 * @returns 読み込み可能な拡張子のベクター
 */
#[tauri::command]
fn fetch_can_read_exts()->Vec<String>{
    image::ImageFormat::all()
        .filter(|f| f.can_read())
        .flat_map(|f| f.extensions_str())
        .copied()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
}

/**
 * 書き込み可能な拡張子を取得する
 * @returns 書き込み可能な拡張子のベクター
 */
#[tauri::command]
fn fetch_can_write_exts()->Vec<String>{
    image::ImageFormat::all()
        .filter(|f| f.can_write())
        .flat_map(|f| f.extensions_str())
        .copied()
        .map(|f| f.to_string())
        .chain(std::iter::once("svg".to_string()))
        .collect::<Vec<String>>()
}

/**
 * 画像変換コマンド
 * @args input: 変換元のファイルパス
 * @args convert_to: 変換後の拡張子
 * @args folder: 保存先フォルダー（空文字の場合は元のフォルダーに保存）
 * @returns 成功した場合はOk、失敗した場合はErrにエラーメッセージを格納して返す
 */
#[tauri::command]
async fn convert_file(input: &str, convert_to: &str, folder: &str) -> Result<(), String> {
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

    // 対応非対応確認
    if !can_read(input) || !can_convert(&new_path){
        return Err("対応していない拡張子です。".to_string());
    } 

    // 画像保存
    let input_str = input.to_string();
    let convert_to_str = convert_to.to_string();
    let dest_path = new_path.clone();

    tauri::async_runtime::spawn_blocking(move || {
        match convert_to_str.as_str() {
            "svg" => {
                convert_by_img2svg_crate(input_str, &dest_path)
            },
            _ => {
                convert_by_image_crate(input_str, &dest_path)
            }
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

/**
 * コマンド引数を取得する
 * @returns コマンド引数のベクター、引数がない場合は空のベクターを返す
 */
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
        .invoke_handler(tauri::generate_handler![convert_file, fetch_args, fetch_can_read_exts, fetch_can_write_exts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
