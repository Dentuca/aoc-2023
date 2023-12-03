use aoc2023::read_input;

fn main() {
    let input = read_input(3);
    let grid = build_grid(&input);
    let part_numbers = find_part_numbers(grid);
    // println!("Part numbers: {:?}", part_numbers);
    let answer: u32 = part_numbers.iter().sum();
    println!("Answer: {}", answer);
}

type Grid<'a> = Vec<&'a [Cell]>;
type Cell = u8;
type Coordinate = (usize, usize);

fn build_grid(input: &str) -> Grid {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn find_part_numbers(grid: Grid) -> Vec<u32> {
    let mut part_numbers = vec![];

    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let mut current_number: Option<u32> = None;
    let mut is_part_number = false;
    for i in 0..rows {
        for j in 0..cols {
            let cell = grid[i][j];
            if let Some(digit) = to_digit(cell) {
                // we found a digit
                if !is_part_number && is_adjacent_to_symbol(&grid, (i, j)) {
                    is_part_number = true;
                }
                let digit = digit as u32;
                current_number = match current_number {
                    Some(current_number) => Some(current_number * 10 + digit),
                    None => Some(digit),
                }
            } else {
                // we didn't find a digit
                if is_part_number {
                    part_numbers.push(current_number.unwrap());
                }
                current_number = None;
                is_part_number = false;
            }
        }
    }

    part_numbers
}

fn to_digit(cell: Cell) -> Option<u8> {
    if cell.is_ascii_digit() {
        Some(cell - b'0')
    } else {
        None
    }
}

fn is_adjacent_to_symbol(grid: &Grid, coord: Coordinate) -> bool {
    let adjacent_coordinates = get_valid_adjacent_coordinates(grid, coord);
    for adjacent_coordinate in adjacent_coordinates.into_iter() {
        if is_coordinate_a_symbol(grid, adjacent_coordinate) {
            return true
        }
    }
    false
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

fn is_coordinate_a_symbol(grid: &Grid, coord: Coordinate) -> bool {
    let cell = grid[coord.0][coord.1];
    is_cell_a_symbol(cell)
}

fn is_cell_a_symbol(cell: Cell) -> bool {
    !cell.is_ascii_digit() && cell != b'.'
}
