use std::fs;

fn read_file(name: &str) -> String
{
    let input = 
        fs::read_to_string(
            format!(
                "{}/{}",
                env!("CARGO_MANIFEST_DIR"),
                name))
        .expect("file not read!");
    input.trim().to_string()
}

fn main() {
    println!("===Day 3!===");

    let input = read_file("example.txt");

    for battery_bank in input.lines() {
        // find largest battery ignoring last slot, if multiple, pick
        // the first index
        // then look for the largest battery after the first index
        // Then combine the two chars and add to running total

        for battery in battery_bank.chars() {
            assert!(('0' <= battery) && (battery <= '9'),
              "Battery not in range! ({battery})");

            
        }
    }
}
