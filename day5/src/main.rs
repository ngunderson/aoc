use std::fs;

fn read_file(name: &str) -> String {
    let input = fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), name))
        .expect("file not read!");
    input.trim().to_string()
}

struct Range {
    first: u64,
    second: u64,
}

impl Range {
    fn contains(&self, num: u64) -> bool {
        return (num >= self.first) && (num <= self.second);
    }
}

fn part1(input: &str) {
    let mut ranges = Vec::new();
    let mut first_half = true;
    let mut fresh_ingredient_count = 0;

    for (_index, line) in input.lines().enumerate() {
        if line.is_empty() {
            first_half = false;
            continue;
        }

        if first_half {
            let numbers: Vec<_> = line.split('-').collect();
            ranges.push(Range {
                first: numbers[0].parse().unwrap(),
                second: numbers[1].parse().unwrap(),
            });
        } else {
            let ingredient_id: u64 = line.parse().unwrap();
            let fresh = ranges.iter().any(|range| range.contains(ingredient_id));
            if fresh {
                fresh_ingredient_count += 1;
            }
        }
    }

    println!("part1, fresh ingredient count: {fresh_ingredient_count}");
}

fn main() {
    println!("day 5!");

    let input = read_file("real.txt");

    part1(&input);
}
