use aoc2023::read_input;

fn main() {
    let input = read_input(4);
    let answer: u32 = input
        .lines()
        .map(Card::from)
        .map(|card| card.points())
        .sum();
    println!("Answer: {}", answer);
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

impl Card {
    fn from(input: &str) -> Self {
        let mut pipe_split = input.split(" | ");

        let mut colon_split = pipe_split.next().unwrap().split(": ");
        colon_split.next();
        let winning_numbers: Vec<u32> = colon_split
            .next().unwrap()
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        let scratched_numbers: Vec<u32> = pipe_split
            .next().unwrap()
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Self {
            winning_numbers,
            scratched_numbers,
        }
    }

    fn points(&self) -> u32 {
        let matches = self.scratched_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as u32;

        if matches == 0 {
            0
        } else {
            2_u32.pow(matches - 1)
        }
    }
}
