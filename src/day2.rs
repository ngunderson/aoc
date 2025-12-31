use std::fs;

fn is_odd(num: i32) -> bool
{
    (num % 2) != 0
}

fn is_valid(id: &String) -> bool {
    // From the description,
    // "...you can find the invalid IDs by looking for any ID
    // which is made only of some sequence of digits repeated twice"

    let id_len: i32 = id.len().try_into().expect("String shouldn't be this long!");

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

pub(crate) fn day2() {
    println!("===Day 2!===");

    let input = fs::read_to_string("day2_sample_input.txt")
        .expect("file not read!");
    let input = input.trim();

    let mut invalid_sum: i32 = 0;

    for range in input.split(",") {
        let range: Vec<i32> = range
            .split("-")
            .map(|s| 
                s
                    .parse()
                    .expect(&format!("Failed to convert! {s}")))
            .collect();

        assert!(range.len() == 2, "Range not size 2!");

        //println!("{} to {}", range[0], range[1]);

        for id in range[0]..=range[1] {
            println!("id {id}");

            let id: String = id.to_string();

            if !is_valid(&id) {
                let id: i32 = id
                    .parse()
                    .expect("conversion back failed!?");

                println!("invalid! {id}");

                invalid_sum = invalid_sum.strict_add(id);
            }
        }
    }
    println!("sum of invalid ids: {invalid_sum}");
}