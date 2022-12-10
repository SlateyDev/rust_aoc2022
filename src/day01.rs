use std::fs;

pub fn run() {
    let file_data = fs::read_to_string("input-day01.txt");

    if file_data.is_ok() {
        // println!("{}", file_data.unwrap());
    }
}