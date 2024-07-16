use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap_or_else(|_| panic!("File not found"))
}
