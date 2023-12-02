use aoc2023::read_input;

fn main() {
    let input = read_input(1);
    let calibration_values = input
        .lines()
        .map_while(|line| -> Option<u32> {
            let left_digit = line.chars().find(char::is_ascii_digit)?.to_digit(10)?;
            let right_digit = line.chars().rfind(char::is_ascii_digit)?.to_digit(10)?;
            println!("Digits are: {}, {}", left_digit, right_digit);
            Some(left_digit * 10 + right_digit)
        });

    let result: u32 = calibration_values.sum();

    println!("Result: {}", result);
}
