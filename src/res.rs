use std::env::current_exe;
use std::path::PathBuf;

pub fn get_input(name: &str) -> PathBuf {
    project_dir().join("input").join(name)
}

pub fn project_dir() -> PathBuf {
    let exe = current_exe().unwrap();
    for a in exe.ancestors() {
        if a.file_name().map(|n| n == "euler-rs").unwrap_or(false) && a.is_dir() {
            return a.to_path_buf();
        }
    }
    panic!();
}
