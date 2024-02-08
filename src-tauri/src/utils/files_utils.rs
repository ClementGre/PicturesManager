use std::path;
use std::path::Path;

pub fn path_to_unix_path_string(path: &Path) -> String {
    if path::MAIN_SEPARATOR != '/' {
        return path.to_str().unwrap().replace(path::MAIN_SEPARATOR_STR, "/");
    }
    path.to_str().unwrap_or("Trying to unwrap non UTF-8 path.").to_string()
}
pub fn path_from_unix_path_string(path: String) -> String {
    if path::MAIN_SEPARATOR != '/' {
        return path.replace("/", path::MAIN_SEPARATOR_STR);
    }
    path
}
