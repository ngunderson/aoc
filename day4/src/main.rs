use std::fs;

fn read_file(name: &str) -> String {
    let input = fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), name))
        .expect("file not read!");
    input.trim().to_string()
}

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

fn get_adjacent_num_rolls(position: (usize, usize), grid: &CharGrid) -> i32 {
    // Check 8 adjacent positions
    0
}

fn main() {
    let input = read_file("example.txt");

    let grid = create_grid(&input);
    let mut part1_roll_count = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if get_adjacent_num_rolls((row_index, column_index), &grid) < 4 {
                part1_roll_count += 1;
            }
            print!("{column}");
        }
        println!();
    }

    println!("part 1 roll count {part1_roll_count}");
}
