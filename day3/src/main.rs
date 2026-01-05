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

fn find_max(battery_bank: &str, previous_index: Option<usize>, digit_position: u8) -> (usize, u32) {
    println!("prev indx {}", previous_index.unwrap_or_default());
    // Start the search right after the previously identified largest number
    // Also ignore the last digits which are smaller than the digit position
    // we're searching for.
    let start_index = if let Some(value) = previous_index {
        value + 1
    } else {
        0
    };
    let last_battery_index = battery_bank.len() - 1;
    // For example, digit position 1, should point at very last element, so
    // last battery index - (1 - 1) = last battery index
    let final_index = last_battery_index - usize::from(digit_position - 1);
    let slice_to_search = &battery_bank[start_index..=final_index];

    let mut max_battery = char::MIN;
    let mut max_battery_index = None;

    // Find the largest char. If there are multiple, use the first index
    // so we can find the largest possible joltage.
    for (index, battery) in slice_to_search.chars().enumerate() {
        if battery > max_battery {
            max_battery = battery;
            max_battery_index = Some(index);
        }
    }

    println!("{} found for {}", max_battery, slice_to_search);
    (
        max_battery_index.unwrap() + start_index,
        max_battery.to_digit(10).unwrap(),
    )
}

fn highest_n_digit_joltage(battery_bank: &str, num_digits: u8) -> u64 {
    assert!(num_digits > 0 && num_digits <= 12, "Size invalid!");

    let mut previous_index = None;
    let mut max_joltage: u64 = 0;

    for digit_position in (1..=num_digits).rev() {
        let (index, value) = find_max(battery_bank, previous_index, digit_position);

        previous_index = Some(index);

        max_joltage = max_joltage + (u64::from(value) * 10_u64.pow(u32::from(digit_position) - 1));
    }
    println!("Joltage found ({max_joltage}) for {battery_bank}");

    max_joltage
}

fn main() {
    println!("===Day 3!===");

    let input = read_file("example.txt");

    let mut part1_sum: u64 = 0;
    let mut part2_sum: u64 = 0;

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

        part1_sum += u64::from(max_joltage);

        part2_sum += highest_n_digit_joltage(battery_bank, 12);
    }

    println!("Part1 sum ({part1_sum})");
    println!("Part2 sum ({part2_sum})");
}
