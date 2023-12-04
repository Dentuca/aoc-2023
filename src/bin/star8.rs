use aoc2023::read_input;

fn main() {
    let input = read_input(4);
    let cards: Vec<Card> = input
        .lines()
        .map(Card::from)
        .collect();
    let answer: u32 = count_scratchcards(&cards);
    println!("Answer: {}", answer);
}

fn count_scratchcards(cards: &[Card]) -> u32 {
    let mut counts = vec![1; cards.len()];
    for (idx, card) in cards.iter().enumerate() {
        for next_idx in idx + 1..idx + 1 + card.points() as usize {
            counts[next_idx] += counts[idx];
        }
    }
    counts.iter().sum()
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
        self.scratched_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as u32
    }
}
