use std::{io::Read, path::Path};

pub fn read(path: &str) -> std::io::Result<impl Read> {
    let path = Path::new("./tests/resources/").join(path);
    std::fs::File::open(path)
}

pub fn read_string(path: &str) -> std::io::Result<String> {
    let path = Path::new("./tests/resources/").join(path);
    std::fs::read_to_string(path)
}
