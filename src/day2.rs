use std::fs;

fn is_invalid(_id: &String) -> bool {
    false
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

            if is_invalid(&id) {
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