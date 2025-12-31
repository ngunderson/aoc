use std::fs;

fn is_odd(num: i64) -> bool
{
    (num % 2) != 0
}

fn is_valid_part1(id: &String) -> bool {
    // From the description,
    // "...you can find the invalid IDs by looking for any ID
    // which is made only of some sequence of digits repeated twice"

    let id_len: i64 = id.len().try_into().expect("String shouldn't be this long!");

    if is_odd(id_len) {
        // Any odd ID, cannot be made of a sequence repeated twice
        // and will never be invalid.
        return true;
    } else {
        let (first, last) = id.split_at(id.len() / 2);

        if first == last {
            return false;
        }
    }

    true
}

fn is_valid_part2(id: &String) -> bool {

    // Iterate through possible digit sequences. The Sequence cannot be
    // greater than half the length since it has to be repeated at least twice.
    for len in 1..=(id.len()/2) {
        let digit_sequence = id.get(0..len).expect("String length invalid!");

        if id.len() % digit_sequence.len() != 0 {
            // If multiples of the digit sequence do not fit in the id,
            // this digit_sequence will not cause the id to be invalid
            continue;
        }

        if id.matches(digit_sequence).count() == (id.len() / digit_sequence.len()) {
            // If the digit sequence is found repeatedly in the id, the id
            // is invalid
            return false;
        }
    }

    true
}

pub(crate) fn day2() {
    println!("===Day 2!===");

    let input = fs::read_to_string("input/day2/real.txt")
        .expect("file not read!");
    let input = input.trim();

    // Some input integers require 64 bit size
    let mut part1_invalid_sum: i64 = 0;
    let mut part2_invalid_sum: i64 = 0;

    for range in input.split(",") {
        let range: Vec<i64> = range
            .split("-")
            .map(|s|
                s
                    .parse()
                    .expect(&format!("Failed to convert! {s}")))
            .collect();

        assert!(range.len() == 2, "Range not size 2!");

        //println!("{} to {}", range[0], range[1]);

        for id in range[0]..=range[1] {
            // println!("id {id}");

            let id: String = id.to_string();

            if !is_valid_part1(&id) {
                let id: i64 = id
                    .parse()
                    .expect("conversion back failed!?");

                //println!("part1 invalid! {id}");

                part1_invalid_sum = part1_invalid_sum.strict_add(id);
            }

            if !is_valid_part2(&id) {
                let id: i64 = id
                    .parse()
                    .expect("conversion back failed!?");

                //println!("part2 invalid! {id}");

                part2_invalid_sum = part2_invalid_sum.strict_add(id);
            }
        }
    }
    println!("sum of part1 invalid ids: {part1_invalid_sum}");
    println!("sum of part2 invalid ids: {part2_invalid_sum}");
}