use std::io::{Read};
use std::fs::File;

pub fn read_to_string(s: &str) -> String {
    let mut file = File::open(s).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}