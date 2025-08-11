use chrono::{DateTime, Local};
use std::time::SystemTime;

pub static DOUBLE_CLICK_DURATION: u128 = 400;

pub fn get_file_extension(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
}

pub fn get_file_icon(filename: &str, is_dir: bool) -> &'static str {
    if is_dir {
        return "📁";
    }

    match get_file_extension(filename) {
        Some(ext) => match ext.to_lowercase().as_str() {
            // Text files:
            "txt" | "md" => "📄",

            // Configuration files:
            "json" | "yaml" | "toml" | "xml" => "⚙️",

            // Code files:
            "rs" | "py" | "c" | "cc" | "cpp" | "go" | "ts" | "js" | "tsx" | "jsx" | "m"
            | "java" | "h" | "hpp" => "💻",
            "html" | "css" | "scss" | "sass" => "🌐",
            "sh" | "bash" | "zsh" | "bat" | "cmd" => "⚡️",

            // Images:
            "jpg" | "png" | "jpeg" | "gif" | "bmp" | "svg" | "ico" | "tiff" | "heic" | "webp" => {
                "🌄"
            }

            // Audio:
            "mp3" | "wav" | "ogg" | "aac" => "🎵",

            // Video:
            "mp4" | "avi" | "mov" | "mkv" | "wmv" | "webm" | "flv" => "🎬",

            // Zips:
            "zip" | "tar" | "gz" | "7z" | "rar" => "📦",

            // Documents:
            "doc" | "docx" => "📘",
            "pdf" => "📕",
            "xslx" | "xls" | "ods" | "csv" => "📗",
            "ppt" | "pptx" => "📙",

            // Executables:
            "exe" | "msi" | "app" => "⚡️",

            _ => "📄",
        },
        None => "📄",
    }
}

pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;

    if size == 0 {
        return "0 B".to_string();
    }

    let mut size_f = size as f64;
    let mut unit_idx = 0;

    while size_f >= (THRESHOLD as f64) && unit_idx < UNITS.len() - 1 {
        size_f /= THRESHOLD as f64;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", size, UNITS[0])
    } else {
        format!("{:.1} {}", size_f, UNITS[unit_idx])
    }
}

pub fn truncate_string(s: &str, max_width: usize) -> String {
    if s.len() <= max_width {
        s.to_string()
    } else if max_width <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &s[..max_width - 3])
    }
}

pub fn format_time(time: SystemTime) -> String {
    let date_time: DateTime<Local> = time.into();
    date_time.format("%Y-%m-%d %H:%M").to_string()
}
