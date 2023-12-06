use aoc2023::read_input;

fn main() {
    let input = read_input(6);
    let race = parse_race(&input);
    let answer: u64 = race.number_of_ways_to_beat_record();
    println!("Answer: {}", answer);
}

fn parse_race(input: &str) -> Race {
    let mut lines = input.lines();

    let time: u64 = lines
        .next().unwrap()
        .rsplit(':')
        .next().unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse().unwrap();

    let distance: u64 = lines
        .next().unwrap()
        .rsplit(':')
        .next().unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse().unwrap();

    Race { time, record: distance }
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn number_of_ways_to_beat_record(&self) -> u64 {
        let mut result = 0;
        for charging_time in 0..self.time {
            let distance = charging_time * (self.time - charging_time);
            if distance > self.record {
                result += 1;
            }
        }
        result
    }
}
