use aoc2023::read_input;

fn main() {
    let input = read_input(2);
    let games = parse_games(&input);
    let answer: u32 = games.iter().map(Game::power).sum();
    println!("Answer: {}", answer);
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(Game::from).collect()
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

    fn power(&self) -> u32 {
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for reveal in &self.reveals {
            max_red = if reveal.red > max_red { reveal.red } else { max_red };
            max_green = if reveal.green > max_green { reveal.green } else { max_green };
            max_blue = if reveal.blue > max_blue { reveal.blue } else { max_blue };
        }

        max_red * max_green * max_blue
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
