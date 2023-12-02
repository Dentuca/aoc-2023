use std::{fs::File, io::Read};

pub fn read_input(day_number: u32) -> String {
    let input_path = format!("inputs/input-{}.txt", day_number);
    let mut contents = String::new();
    File::open(input_path).unwrap().read_to_string(&mut contents).unwrap();
    contents
}
