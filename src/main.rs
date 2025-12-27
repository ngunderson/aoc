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

struct DialPt2
{
    position : i32,
    zero_count : i32,
}

impl DialPt2 {
    fn new() -> DialPt2 {
        return DialPt2 {
            position : 50,
            zero_count : 0,
        };
    }

    const DIAL_POSITIONS : i32 = 100;

    fn rotate(&mut self, amount : i32) {
        assert!(amount != 0, "amount 0!");

        let prev_position = self.position;

        // Update the dial position
        self.position = (self.position + amount) % Self::DIAL_POSITIONS;

        if self.position < 0 {
            self.position = Self::DIAL_POSITIONS + self.position;
        }

        // Increment zero count for each full rotation of the dial
        self.zero_count += amount / Self::DIAL_POSITIONS;

        // Calculate the remaining amount to move the dial
        let remaining_amount = amount % Self::DIAL_POSITIONS;

        // If the remainder moves the dial to or past 0 in either direction, 
        // increment the zero count
        if (prev_position + remaining_amount) >= Self::DIAL_POSITIONS {
            self.zero_count += 1;
        } else if (prev_position + remaining_amount) <= 0 {
            self.zero_count += 1;
        } else {
            // value did not pass 0, do nothing
        }

        //println!("amount({}), prev({}), new({}), cnt({})", amount, prev_position, self.position, self.zero_count);
    }
}

fn main() {
    println!("Hello, advent of code!");

    let rotation_codes = fs::read_to_string("day1_input.txt")
        .expect("file not read!");

    let mut dial = Dial::new();
    let mut dial_pt2 = DialPt2::new();
    let mut result = 0;

    for (index, line) in rotation_codes.lines().enumerate() {
        if line.starts_with("R") {
            let right_code: i32 = line[1..].parse().expect(&format!("code not found! {index}"));
            dial.rotate(right_code);
            dial_pt2.rotate(right_code);
        } else if line.starts_with("L") {
            let left_code: i32 = line[1..].parse().expect(&format!("code not found! {index}"));
            dial.rotate(-left_code);
            dial_pt2.rotate(-left_code);
        } else if line.is_empty() {
            // ignore empty
        } else {
            panic!("Invalid format on line {}", index);
        }

        if dial.position == 0 {
            result += 1;
        }
    }

    println!("part 1 result is {}", result);
    println!("part 2 result is {}", dial_pt2.zero_count);

}
