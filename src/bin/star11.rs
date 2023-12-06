use aoc2023::read_input;

fn main() {
    let input = read_input(6);
    let races = parse_races(&input);
    let answer: u32 = races
        .iter()
        .fold(1, |acc, race| acc * race.number_of_ways_to_beat_record());
    println!("Answer: {}", answer);
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();

    let times: Vec<u32> = lines
        .next().unwrap()
        .rsplit(':')
        .next().unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let distances: Vec<u32> = lines
        .next().unwrap()
        .rsplit(':')
        .next().unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, record: distance })
        .collect()
}

struct Race {
    time: u32,
    record: u32,
}

impl Race {
    fn number_of_ways_to_beat_record(&self) -> u32 {
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
