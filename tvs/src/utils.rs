#![allow(dead_code)]

/// List directory [files only] and filter them with list of extentions
pub fn filter_path_w_ext(dir: &str, exts: &[&str]) -> Vec<String> {
    std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| {
            if path.extension().map_or(
                false,
                |ext| { exts.iter().any(|x| *x == ext.to_ascii_lowercase()) } && path.is_file(),
            ) {
                Some(path)
            } else {
                None
            }
        })
        .map(|r| String::from(r.file_name().unwrap().to_str().unwrap()))
        .collect::<Vec<_>>()
}

/// List all files and directories in `dir`
pub fn list_paths(dir: &str) -> Vec<String> {
    std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .map(|r| String::from(r.file_name().unwrap().to_str().unwrap()))
        .collect::<Vec<_>>()
}

pub fn colored_rgb(r: i32, g: i32, b: i32, text: &str) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
}

pub fn colored(c: i32, text: &str) -> String {
    format!("\x1B[1;{}m{}\x1B[0m", c, text)
}
