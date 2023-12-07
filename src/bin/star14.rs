use std::cmp::Ordering;

use aoc2023::read_input;

fn main() {
    let input = read_input(7);

    let mut hands: Vec<Hand> = input
        .lines()
        .map(Hand::from)
        .collect();

    hands.sort();

    let answer: u32 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i as u32 + 1) * hand.bid));

    println!("Answer: {}", answer);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Unknown card {}", c),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn from(s: &str) -> Self {
        let mut split = s.split_whitespace();
        let cards = split.next().unwrap().chars().map(Card::from).collect();
        let bid: u32 = split.next().unwrap().parse().unwrap();
        Hand { cards, bid }
    }

    fn rank(&self) -> HandRank {
        let mut card_counts: Vec<(Card, u32)> = Vec::new();
        for card in &self.cards {
            match card_counts.iter_mut().find(|(c, _)| c == card) {
                Some(card_count) => card_count.1 += 1,
                None => card_counts.push((card.clone(), 1)),
            }
        }

        card_counts.sort_by(|(card1, count1), (card2, count2)|
            match count1.cmp(count2) {
                Ordering::Equal => card1.cmp(card2),
                order => order,
            }
        );

        if card_counts.len() != 1 {
            let j_count = card_counts
                .iter_mut()
                .find(|(card, _)| *card == Card::J)
                .unwrap_or(&mut (Card::J, 0))
                .1;
            card_counts.retain(|(card, _)| *card != Card::J);
            card_counts.last_mut().unwrap().1 += j_count;
        }

        // this strategy assumes the hand contains exactly 5 cards
        match card_counts.len() {
            1 => HandRank::FiveOfAKind,
            2 => match card_counts.last().unwrap().1 {
                3 => HandRank::FullHouse,
                _ => HandRank::FourOfAKind,
            },
            3 => match card_counts.last().unwrap().1 {
                2 => HandRank::TwoPair,
                _ => HandRank::ThreeOfAKind,
            },
            4 => HandRank::OnePair,
            _ => HandRank::HighCard,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() {
            return false
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a != b {
                    return false
                }
            }
        }
        true
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank().cmp(&other.rank()) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        order => { return order },
                    }
                }
                Ordering::Equal
            },
            rank => rank,
        }
    }
}
