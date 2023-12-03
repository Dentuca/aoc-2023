use std::collections::{HashMap, HashSet};

use aoc2023::read_input;

fn main() {
    let input = read_input(3);
    let grid = build_grid(&input);
    let part_numbers = find_gear_ratios(grid);
    let answer: u32 = part_numbers.iter().sum();
    println!("Answer: {}", answer);
}

type Grid<'a> = Vec<&'a [Cell]>;
type Cell = u8;
type Coordinate = (usize, usize);

fn build_grid(input: &str) -> Grid {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn find_gear_ratios(grid: Grid) -> Vec<u32> {
    let mut potential_gears: HashMap<Coordinate, Vec<u32>> = HashMap::new();

    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let mut current_number: Option<u32> = None;
    let mut adjacent_stars: HashSet<Coordinate> = HashSet::new();
    for i in 0..rows {
        for j in 0..cols {
            let cell = grid[i][j];
            if let Some(digit) = to_digit(cell) {
                // we found a digit
                for adjacent_star in get_adjacent_star_symbols(&grid, (i, j)) {
                    adjacent_stars.insert(adjacent_star);
                }
                let digit = digit as u32;
                current_number = match current_number {
                    Some(current_number) => Some(current_number * 10 + digit),
                    None => Some(digit),
                }
            } else {
                // we didn't find a digit
                if !adjacent_stars.is_empty() {
                    for adjacent_star in adjacent_stars.iter() {
                        if !potential_gears.contains_key(adjacent_star) {
                            potential_gears.insert(*adjacent_star, vec![]);
                        }
                        potential_gears
                            .get_mut(adjacent_star)
                            .unwrap()
                            .push(current_number.unwrap());
                    }
                }
                current_number = None;
                adjacent_stars.clear();
            }
        }
    }

    potential_gears
        .values()
        .filter(|part_numbers| part_numbers.len() == 2)
        .map(|part_numbers| part_numbers[0] * part_numbers[1])
        .collect()
}

fn to_digit(cell: Cell) -> Option<u8> {
    if cell.is_ascii_digit() {
        Some(cell - b'0')
    } else {
        None
    }
}

fn get_adjacent_star_symbols(
    grid: &Grid,
    coord: Coordinate,
) -> Vec<Coordinate> {
    get_valid_adjacent_coordinates(grid, coord)
        .into_iter()
        .filter(|adjacent_coord|
            is_coordinate_a_star_symbol(grid, *adjacent_coord)
        )
        .collect()
}

fn get_valid_adjacent_coordinates(
    grid: &Grid,
    coord: Coordinate,
) -> Vec<Coordinate> {
    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let mut valid_adjacent_coordinates = vec![];

    // upper row (could not exist)
    if let Some(upper_row) = coord.0.checked_sub(1) {
        if let Some(left_col) = coord.1.checked_sub(1) {
            valid_adjacent_coordinates.push((upper_row, left_col));

            // remaining left column since we already unwrapped it
            valid_adjacent_coordinates.push((coord.0, left_col));
            if coord.0 + 1 < rows {
                valid_adjacent_coordinates.push((coord.0 + 1, left_col));
            }
        }
        valid_adjacent_coordinates.push((upper_row, coord.1));
        if coord.1 + 1 < cols {
            valid_adjacent_coordinates.push((upper_row, coord.1 + 1));
        }
    }

    // bottom row (except for bottom-left, since we already added it)
    if coord.0 + 1 < rows {
        valid_adjacent_coordinates.push((coord.0 + 1, coord.1));
        if coord.1 + 1 < cols {
            valid_adjacent_coordinates.push((coord.0 + 1, coord.1 + 1));
        }
    }

    // only missing immediate right
    if coord.1 + 1 < cols {
        valid_adjacent_coordinates.push((coord.0, coord.1 + 1));
    }

    valid_adjacent_coordinates
}

fn is_coordinate_a_star_symbol(grid: &Grid, coord: Coordinate) -> bool {
    let cell = grid[coord.0][coord.1];
    is_cell_a_star_symbol(cell)
}

fn is_cell_a_star_symbol(cell: Cell) -> bool {
    cell == b'*'
}
