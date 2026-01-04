use std::fs;

fn read_file(name: &str) -> String {
    let input = fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), name))
        .expect("file not read!");
    input.trim().to_string()
}

fn first_battery(battery_bank: &str) -> (char, usize) {
    let mut max_char: char = char::MIN;
    let mut first_battery = None;
    let mut bank_chars = battery_bank.chars();

    // The first battery of the two required
    // cannot be the last index
    _ = bank_chars.next_back();

    // We intentionally want the first index if multiple are
    // equal to give ensure the second battery is the largest possible.
    // .max() not used because it returns the last index if multiple
    // match.
    for (index, battery) in bank_chars.enumerate() {
        if battery > max_char {
            max_char = battery;
            first_battery = Some(index);
        }
    }
    (max_char, first_battery.expect("Battery not found!"))
}

fn second_battery(battery_bank: &str, first_index: usize) -> char {
    let second_battery = battery_bank[(first_index + 1)..].chars().max();

    second_battery.expect("Second battery not found!")
}

fn main() {
    println!("===Day 3!===");

    let input = read_file("real.txt");

    let mut part1_sum = 0;

    for battery_bank in input.lines() {
        // find largest battery ignoring last slot, if multiple, pick
        // the first index
        // then look for the largest battery after the first index
        // Then combine the two chars and add to running total

        let (first_val, first_index) = first_battery(battery_bank);
        let second_battery = second_battery(battery_bank, first_index);

        let max_joltage = (first_val.to_digit(10).expect("first invalid!") * 10)
            + second_battery.to_digit(10).expect("second invalid!");

        //println!("line is ({battery_bank}), max ({max_joltage})");

        part1_sum += max_joltage;
    }

    println!("Part1 sum ({part1_sum})");
}
