use std::fs;

struct Dial
{
    position : i32,
}

impl Dial {
    fn new() -> Dial {
        return Dial {
            position : 50
        };
    }

    const DIAL_POSITIONS : i32 = 100;

    fn rotate(&mut self, amount : i32) {
        self.position = (self.position + amount) % Self::DIAL_POSITIONS;

        if self.position < 0 {
            self.position = Self::DIAL_POSITIONS + self.position;
        }        
    }
}

// Return 1 if the dial is positioned at 0
fn part1_strategy_iteration(dial: &Dial) -> i32 {
    if dial.position == 0 {
        1
    } else {
        0
    }
}

// Count the number of times the amount causes the dial
// to point to 0 at an point during the rotation.
fn part2_strategy_iteration(dial: &Dial, amount : i32) -> i32 {
    assert!(amount != 0, "amount 0!");

    let mut zero_count = 0;

    // Increment zero count for each full rotation of the dial
    zero_count += amount.abs() / Dial::DIAL_POSITIONS;

    // Calculate the remaining amount to see if the dial passes 0 more
    let remaining_amount = amount % Dial::DIAL_POSITIONS;

    if dial.position == 0 {
        // If we currently sit at 0, the remainder, which we know is less
        // than 100, will no make us reach zero again.
    } else if remaining_amount == 0 {
        // do nothing with no remainder
    } else if remaining_amount > 0 
        && ((dial.position + remaining_amount) >= Dial::DIAL_POSITIONS) {
        // Positive remaining amount went past or to 0
        zero_count += 1;
    } else if remaining_amount < 0 
        && (dial.position + remaining_amount) <= 0 {
        // Similar check is (remaining_amount.abs() >= dial.position)
        // Negative remaining amount, increment for scenarios
        // where 0 is reached or passed
        // Position 0, L20, prev + remainder = -20
        // Position 10, L20, prev + remainder = -10
        // Position 20, L20, prev + remainder = 0
        zero_count += 1;
    }

    //println!("amount({}), prev({}), new({}), cnt({})", amount, prev_position, self.position, self.zero_count);

    zero_count
}



fn main() {
    println!("Hello, advent of code!");

    let rotation_codes = fs::read_to_string("day1_input.txt")
        .expect("file not read!");

    let mut dial = Dial::new();
    let mut result_pt1 = 0;
    let mut result_pt2 = 0;

    for (index, line) in rotation_codes.lines().enumerate() {
        #[allow(unused_assignments)]
        let mut amount_to_rotate = 0;
        if line.starts_with("R") {
            let right_code: i32 = line[1..].parse().expect(&format!("code not found! {index}"));
            amount_to_rotate = right_code;          
        } else if line.starts_with("L") {
            let left_code: i32 = line[1..].parse().expect(&format!("code not found! {index}"));
            amount_to_rotate = -left_code;
        } else if line.is_empty() {
            // ignore empty
            continue;
        } else {
            panic!("Invalid format on line {}", index);
        }

        // Count part 2 before we rotate the dial's position
        result_pt2 += part2_strategy_iteration(&dial, amount_to_rotate);

        dial.rotate(amount_to_rotate);

        result_pt1 += part1_strategy_iteration(&dial);
    }

    println!("part 1 result is {}", result_pt1);
    println!("part 2 result is {}", result_pt2);

}
