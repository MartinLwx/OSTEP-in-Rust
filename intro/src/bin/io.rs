use std::fs;
use std::io::prelude::*;

fn main() {
    let mut file = fs::File::options()
        .write(true)
        .create(true)
        .open("/tmp/file")
        .unwrap();
    file.write_all(b"Hello world\n").unwrap();
}
