use aoc2023::read_input;

fn main() {
    let input = read_input(2);
    let games = parse_games(&input);
    // println!("Parsed {} games.", games.len());
    // games.iter().for_each(|game| println!("{:#?}", game));
    let valid_games = games.iter().filter(|game| is_valid_game(game, 12, 13, 14));
    let valid_game_ids = valid_games.map(|game| game.id);
    let answer: u32 = valid_game_ids.sum();
    println!("Answer: {}", answer);
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(Game::from).collect()
}

fn is_valid_game(game: &Game, red: u32, green: u32, blue: u32) -> bool {

    for reveal in &game.reveals {
        if !is_valid_reveal(reveal, red, green, blue) {
            return false;
        }
    }

    true
}

fn is_valid_reveal(reveal: &Reveal, red: u32, green: u32, blue: u32) -> bool {
    reveal.red <= red && reveal.green <= green && reveal.blue <= blue
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn from(s: &str) -> Self {

        let mut tokens = s                      // "Game 32: 10 red, 3 green..."
            .split_whitespace();                // ["Game", "32:", "10", "red", ...]
        tokens.next();                          // "Game"
        let id_token = tokens.next().unwrap();  // "32:"
        let id: u32 = id_token.split(':')       // ["32", ""]
            .next().unwrap()                    // "32"
            .parse().unwrap();                  // 32

        let reveals = s         // "Game 32: 10 red, 3 green...; 4 red..."
            .rsplit(": ")       // ["10 red, 3 green...; 4 red...", "Game 32"]
            .next().unwrap()    // "10 red, 3 green...; 4 red..."
            .split(';')         // ["10 red, 3 green...", "4 red..."]
            .map(Reveal::from)  // [Reveal(10, 3, ...), Reveal(4, ...)]
            .collect();

        Game {
            id,
            reveals,
        }
    }
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn from(s: &str) -> Self {
        let mut reveal = Self { red: 0, green: 0, blue: 0 };

        fn token_to_u32(token: &str) -> u32 {
            token.split_whitespace().next().unwrap().parse().unwrap()
        }

        s.split(", ").for_each(|token|
            if token.ends_with("red") { reveal.red = token_to_u32(token) }
            else if token.ends_with("green") { reveal.green = token_to_u32(token) }
            else if token.ends_with("blue") { reveal.blue = token_to_u32(token) }
        );

        reveal
    }
}
