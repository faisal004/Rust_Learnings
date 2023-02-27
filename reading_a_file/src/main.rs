use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file=File::open("intro.txt").expect("cannot open the file");
    let mut contents=String::new();
    file.read_to_string(&mut contents).expect("OOps cannot read file");
    print!("file contents:\n\n {}",contents);
}
