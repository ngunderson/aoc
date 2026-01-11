use std::fs;

fn read_file(name: &str) -> String {
    let input = fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), name))
        .expect("file not read!");
    input.trim().to_string()
}

const PAPER_ROLL: char = '@';

type CharGrid = Vec<Vec<char>>;

fn create_grid(input: &String) -> CharGrid {
    let num_rows: usize = input.lines().count();
    let num_columns = input.lines().next().unwrap().len();

    let mut grid = vec![vec!['.'; num_columns]; num_rows];

    for (row_index, line) in input.lines().enumerate() {
        assert!(line.len() == num_columns, "Line is a different length!");

        for (column_index, c) in line.chars().enumerate() {
            grid[row_index][column_index] = c;
        }
    }

    grid
}

fn get_adjacent_num_rolls((row, col): (usize, usize), grid: &CharGrid) -> u32 {
    // Check 8 adjacent positions

    // Start 1 index less than current, unless position is at an edge
    // End 1 index less than current, unless position is at an edge, + 2 because
    // the subsequent loop iterates to < end position.
    let row_start: usize = if row > 0 { row - 1 } else { 0 };
    let row_end: usize = if row < (grid.len() - 1) {
        row + 2
    } else {
        grid.len()
    };
    let col_start: usize = if col > 0 { col - 1 } else { 0 };
    let col_end: usize = if col < (grid[0].len() - 1) {
        col + 2
    } else {
        grid[0].len()
    };

    let mut adjacent_rolls = 0u32;

    for row_index in row_start..row_end {
        for col_index in col_start..col_end {
            if row_index == row && col_index == col {
                // Only count rolls in adjacent locations, not the passed position
                continue;
            }
            if grid[row_index][col_index] == PAPER_ROLL {
                adjacent_rolls += 1;
            }
        }
    }

    // println!(
    //     "row_start: ({row_start}), row_end: ({row_end}), col_start: ({col_start}), col_end: ({col_end}), adjacent: ({adjacent_rolls})"
    // );

    adjacent_rolls
}

fn main() {
    let input = read_file("example.txt");

    let grid = create_grid(&input);
    let mut part1_accessible_roll_count = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            if *value == PAPER_ROLL && get_adjacent_num_rolls((row_index, column_index), &grid) < 4
            {
                part1_accessible_roll_count += 1;
            }
            print!("{value}");
        }
        println!();
    }

    println!("part 1 roll count {part1_accessible_roll_count}");
}
