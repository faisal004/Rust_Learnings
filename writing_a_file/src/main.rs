use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file=File::create("output.txt").expect("Could not create file!");
    file.write_all(b"I am writing to this file from my rust code").expect("Cannot write to file");
}
