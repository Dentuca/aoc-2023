use aoc2023::read_input;

fn main() {
    let input = read_input(5);
    let almanac = Almanac::from(&input);
    let locations = almanac.map_seeds_to_location();
    let answer = locations.into_iter().min().unwrap();
    println!("Answer: {}", answer);
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();

        let seeds: Vec<u64> = lines
            .next().unwrap()
            .rsplit(": ")
            .next().unwrap()
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

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
            seeds,
            maps
        }
    }

    fn map_seeds_to_location(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed|
                self.maps
                    .iter()
                    .fold(*seed, |acc, map|
                        map.map(acc)
                    ))
            .collect()
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
