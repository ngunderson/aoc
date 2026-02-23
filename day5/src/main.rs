use std::fs;

fn read_file(name: &str) -> String {
    let input = fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), name))
        .expect("file not read!");
    input.trim().to_string()
}

#[derive(Clone, PartialEq, Debug)]
struct Range {
    first: u64,
    second: u64,
}

impl Range {
    fn new(first: u64, second: u64) -> Range {
        Range { first, second }
    }

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

fn set_intersects(a: &Range, b: &Range) -> bool {
    // a is within b or b is within a
    a.contains(b.first) || a.contains(b.second) || b.contains(a.first) || b.contains(a.second)
}

// A \ B
// Return the items in `a` that aren't in `b`
fn set_difference(a: &Range, b: &Range) -> Vec<Range> {
    // No intersection, return a
    if !set_intersects(&a, &b) {
        return vec![a.clone()];
    } else {
        // set a matches set b, return empty
        if a == b {
            return Vec::new();
        }
        // set a is within b, return empty
        else if b.contains(a.first) && b.contains(a.second) {
            return Vec::new();
        }
        // set a overlaps start of b
        // [1, 3] \ [2, 4] = [1]
        // [1, 4] \ [3, 4] = [1, 2]
        // [1, 8] \ [5, 20] = [1, 4]
        else if !b.contains(a.first) && b.contains(a.second) {
            return vec![Range {
                first: a.first,
                second: b.first - 1,
            }];
        }
        // set a overlaps the end of b
        // [2, 4] \ [1, 3] = [4]
        // [5, 10] \ [1, 7] = [8, 10]
        else if b.contains(a.first) && !b.contains(a.second) {
            return vec![Range {
                first: b.second + 1,
                second: a.second,
            }];
        }
        // set b is within set a matching first
        // [2, 8] \ [2, 4] = [5, 8]
        else if a.first == b.first && a.contains(b.second) {
            return vec![Range {
                first: b.second + 1,
                second: a.second,
            }];
        }
        // set b is within set a matching second
        // [2, 8] \ [6, 8] = [2, 5]
        else if a.second == b.second && a.contains(b.first) {
            return vec![Range {
                first: a.first,
                second: b.first - 1,
            }];
        }
        // [2, 8] \ [4, 6] = [2, 3], [7, 8]
        else if a.contains(b.first) && a.contains(b.second) {
            return vec![
                Range {
                    first: a.first,
                    second: b.first - 1,
                },
                Range {
                    first: b.second + 1,
                    second: a.second,
                },
            ];
        } else {
            panic!("Not expected! Ranges are {a:?}, {b:?}");
        }
    }
}

fn range_set_difference(a: &Vec<Range>, b: &Range) -> Vec<Range> {
    let mut new_vec = Vec::new();
    for range in a {
        let difference = set_difference(range, b);

        for range in difference {
            new_vec.push(range);
        }
    }

    new_vec
}

fn main() {
    println!("day 5!");

    let input = read_file("real.txt");

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
            // for each unique range
            //   range \ unique range
            let mut range_vec = Vec::new();
            range_vec.push(range);
            for unique_range in &unique_ranges {
                range_vec = range_set_difference(&range_vec, &unique_range);

                // If the range has no unique elements left, stop
                // comparing against the other unique ranges
                if range_vec.is_empty() {
                    break;
                }
            }
            for range in range_vec {
                unique_ranges.push(range);
            }
        }
    }

    let mut sum_of_ids: u64 = 0;
    for range in unique_ranges {
        sum_of_ids = sum_of_ids + range.len();
    }

    println!("part2, fresh ingredient ids: {sum_of_ids}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_intersection() {
        let result = set_intersects(&Range::new(1, 2), &Range::new(3, 4));

        assert_eq!(result, false);
    }

    #[test]
    fn intersection1() {
        let result = set_intersects(&Range::new(3, 4), &Range::new(3, 4));
        assert_eq!(result, true);
    }

    #[test]
    fn intersection2() {
        let result = set_intersects(&Range::new(1, 4), &Range::new(4, 7));
        assert_eq!(result, true);
    }

    #[test]
    fn intersection3() {
        let result = set_intersects(&Range::new(3, 8), &Range::new(4, 7));
        assert_eq!(result, true);
    }

    #[test]
    fn intersection4() {
        let result = set_intersects(&Range::new(8, 10), &Range::new(9, 11));
        assert_eq!(result, true);
    }

    #[test]
    fn set_difference1() {
        let result = set_difference(&Range::new(8, 10), &Range::new(9, 10));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Range::new(8, 8));
    }

    #[test]
    fn set_difference2() {
        let result = set_difference(&Range::new(8, 10), &Range::new(8, 9));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Range::new(10, 10));
    }

    #[test]
    fn set_difference3() {
        let result = set_difference(&Range::new(8, 12), &Range::new(6, 10));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Range::new(11, 12));
    }

    #[test]
    fn set_difference4() {
        let result = set_difference(&Range::new(8, 12), &Range::new(10, 14));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Range::new(8, 9));
    }

    #[test]
    fn set_difference5() {
        let result = set_difference(&Range::new(8, 12), &Range::new(7, 13));

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn set_difference6() {
        let result = set_difference(&Range::new(8, 12), &Range::new(8, 12));

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn set_difference7() {
        let result = set_difference(&Range::new(7, 12), &Range::new(9, 10));

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Range::new(7, 8));
        assert_eq!(result[1], Range::new(11, 12));
    }

    #[test]
    fn range_set_difference1() {
        let result = range_set_difference(
            &vec![Range::new(3, 7), Range::new(8, 10)],
            &Range::new(5, 7),
        );

        assert_eq!(result, vec![Range::new(3, 4), Range::new(8, 10)]);
    }

    #[test]
    fn range_set_difference2() {
        let result = range_set_difference(
            &vec![Range::new(3, 7), Range::new(8, 10)],
            &Range::new(2, 11),
        );

        assert_eq!(result.len(), 0);
    }
}
