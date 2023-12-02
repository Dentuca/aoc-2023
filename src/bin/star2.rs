use aoc2023::read_input;

const WRITTEN_DIGITS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn main() {
    let result: u32 = read_input(1)
        .lines()
        .filter_map(get_calibration_value)
        .sum();

    println!("Result: {}", result);
    // println!("Result: {}", get_calibration_value("9dsninefive6lhjpdkpcr").unwrap());
}

fn get_calibration_value(line: &str) -> Option<u32> {

    let left_first_written_digit = first_written_digit(line, true);
    let left_first_digit_index = line.find(|c :char| c.is_ascii_digit());
    let left_digit = match (left_first_written_digit, left_first_digit_index) {
        (
            Some((first_written_digit_index, first_written_digit)),
            Some(first_digit_index),
        ) => if first_written_digit_index < first_digit_index {
                first_written_digit
            } else {
                line.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap()
            },
        (
            Some((_, first_written_digit)),
            None,
        ) => first_written_digit,
        (
            None,
            Some(_),
        ) => line.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap(),
        (
            None,
            None,
        ) => return None
    };

    let right_first_written_digit = first_written_digit(line, false);
    let right_first_digit_index = line.rfind(|c :char| c.is_ascii_digit());
    let right_digit = match (right_first_written_digit, right_first_digit_index) {
        (
            Some((first_written_digit_index, first_written_digit)),
            Some(first_digit_index),
        ) => if first_written_digit_index > first_digit_index {
                // println!("first_written_digit_index: {}", first_written_digit_index);
                // println!("first_digit_index: {}", first_digit_index);
                first_written_digit
            } else {
                line.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap()
            },
        (
            Some((_, first_written_digit)),
            None,
        ) => first_written_digit,
        (
            None,
            Some(_),
        ) => line.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap(),
        (
            None,
            None,
        ) => return None
    };

    // println!("Digits for '{}' are: {}, {}", line, left_digit, right_digit);
    // failing for 9dsninefive6lhjpdkpcr, giving 9,5 instead of 9,6

    Some(left_digit * 10 + right_digit)
}

fn first_written_digit(line: &str, left: bool) -> Option<(usize, u32)> {
    WRITTEN_DIGITS
        .iter()
        .enumerate()
        .filter_map(|(i, written_digit)|
            match left {
                true => Some((line.find(written_digit)?, i as u32 + 1)),
                false => Some((line.rfind(written_digit)?, i as u32 + 1))
            }
        )
        .min_by_key(|&(written_digit_index, wd)| {
            // println!("Written digit: {} with index {}", wd, written_digit_index);
            match left {
                true => written_digit_index as i32,
                false => -(written_digit_index as i32)
            }
        })
}
