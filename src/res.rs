use std::env::current_exe;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

#[macro_export]
macro_rules! infile {
    () => {
        euler_rs::res::input_file(file!())
    };
}

pub fn input_file(src_file: &str) -> String {
    let pb = PathBuf::from(src_file);
    let name: String = pb.file_name().unwrap().to_string_lossy().to_string();
    let num = name.split_once('e').unwrap().1.split_once(".").unwrap().0;
    for path in read_dir(project_dir().join("input")).unwrap() {
        let p = path.unwrap().path();
        if p.file_name().unwrap().to_str().unwrap().contains(num) {
            return read_to_string(p).unwrap();
        }
    }
    panic!();
}

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
