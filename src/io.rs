use std::io;
use std::fs::File;
use std::io::prelude::*;

pub fn read_one_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn read_int_arrays_from_file(path: String) -> Vec<Vec<u32>> {
    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.lines().map(|line| line.split_whitespace().map(|w| u32::from_str_radix(w, 10).unwrap()).collect()).collect()
}

pub fn read_string_arrays_from_file(path: String) -> Vec<Vec<String>> {
    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.lines().map(|line| line.split_whitespace().map(|w| String::from(w)).collect()).collect()
}
