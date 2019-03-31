use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", read_username_from_file().expect("OHNO"));
}
