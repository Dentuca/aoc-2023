use aoc2023::read_input;

// WARN: this one takes over 15 mins to compute on my machine
fn main() {
    let input = read_input(5);
    let almanac = Almanac::from(&input);
    let answer = almanac.min_location_for_initial_seeds();
    println!("Answer: {}", answer);
}

type SeedMapping = (u64, u64);

struct Almanac {
    seed_mappings: Vec<SeedMapping>,
    maps: Vec<Map>,
}

impl Almanac {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();

        let mut seed_mappings_split = lines
            .next().unwrap()
            .rsplit(": ")
            .next().unwrap()
            .split_whitespace();

        let mut seed_mappings = Vec::new();
        while let Some(seed_start_str) = seed_mappings_split.next() {
            let seed_start: u64 = seed_start_str.parse().unwrap();
            let length: u64 = seed_mappings_split.next().unwrap().parse().unwrap();
            seed_mappings.push((seed_start, length));
        }

        lines.next();
        lines.next();

        let mut maps: Vec<Map> = Vec::new();
        let mut current_map: Map = Map::new();

        loop {
            match lines.next() {
                Some(line) => {
                    if line.is_empty() {
                        maps.push(current_map);
                        current_map = Map::new();
                        lines.next();
                    } else {
                        let mut split = line.split_whitespace();
                        let dest_start: u64 = split.next().unwrap().parse().unwrap();
                        let source_start: u64 = split.next().unwrap().parse().unwrap();
                        let range_length: u64 = split.next().unwrap().parse().unwrap();
                        current_map.mappings.push(
                            Mapping {
                                source_start,
                                dest_start,
                                range_length,
                            }
                        );
                    }
                },
                None => {
                    maps.push(current_map);
                    break
                }
            }
        }

        Self {
            seed_mappings,
            maps
        }
    }

    fn min_location_for_initial_seeds(&self) -> u64 {
        let mut min_location = u64::MAX;
        self.seed_mappings
            .iter()
            .for_each(|(seed_start, length)| {
                // here I'm being lazy by considering every possible seed
                for seed in *seed_start..*seed_start + length - 1 {
                    let location = self.maps
                        .iter()
                        .fold(seed, |acc, map|
                            map.map(acc)
                        );
                    min_location = if location < min_location {
                        location
                    } else {
                        min_location
                    }
                }
            });
        min_location
    }
}

struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    fn map(&self, source: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(dest) = mapping.map(source) {
                return dest
            }
        }
        source
    }
}

struct Mapping {
    source_start: u64,
    dest_start: u64,
    range_length: u64,
}

impl Mapping {
    fn map(&self, source: u64) -> Option<u64> {
        let source_end = self.source_start + self.range_length - 1;
        if source >= self.source_start && source <= source_end {
            return Some(self.dest_start + (source - self.source_start))
        }
        None
    }
}
