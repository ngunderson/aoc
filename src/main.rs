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

    fn right_rotate(&mut self, amount : i32) {
        self.position = (self.position + amount) % Self::DIAL_POSITIONS;
    }

    fn left_rotate(&mut self, amount : i32) {
        self.position = (self.position - amount) % Self::DIAL_POSITIONS;

        if self.position < 0 {
            self.position = Self::DIAL_POSITIONS + self.position;
        }
    }
}

fn main() {
    println!("Hello, advent of code!");

    let rotation_codes = fs::read_to_string("day1_input.txt")
        .expect("file not read!");

    let mut dial = Dial::new();
    let mut result = 0;

    for (index, line) in rotation_codes.lines().enumerate() {
        if line.starts_with("R") {
            let right_code: i32 = line[1..].parse().expect(&format!("code not found! {index}"));
            dial.right_rotate(right_code);
        } else if line.starts_with("L") {
            let left_code: i32 = line[1..].parse().expect(&format!("code not found! {index}"));
            dial.left_rotate(left_code);
        } else if line.is_empty() {
            // ignore empty
        } else {
            panic!("Invalid format on line {}", index);
        }

        if dial.position == 0 {
            result += 1;
        }
    }

    println!("result is {}", result);

}
