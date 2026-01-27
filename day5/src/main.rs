use std::fs;
use std::collections::HashSet;

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

    fn len(&self) -> u64 {
       if self.first == self.second {
          return 1;
       }
       self.second - self.first + 1
    }
}

// A \ B
// Return the items in `a` that aren't in `b`
fn set_difference(a: &Range, b: &Range) -> Option<Range> {
   // options.. 
   // - no overlap, return a
   // -
   // [1, 3] \ [5, 6] = [1, 3]
   // [1, 3] \ [2, 4] = [1]
   // [1, 4] \ [3, 4] = [1, 2]
   // [6, 9] \ [3, 7] = [8, 9]
   None
}

fn main() {
    println!("day 5!");

    let input = read_file("example.txt");

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

    let mut unique_ranges: Vec<Range> = Vec::new();

    for range in ranges {
    	if unique_ranges.len() == 0 {
	   unique_ranges.push(range);
	} else {
	   
	}
    }

    println!("part2, fresh ingredient ids: ");
}
