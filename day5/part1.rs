use std::fs;
use std::process;

fn main() {
    let mut num_fresh_ingredients: u64 = 0;

    let mut spoiled_ingredients = false;
    let mut fresh_ingredient_ranges: Vec<(u64, u64)> = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        if line == "" {
            spoiled_ingredients = true;
        } else if !spoiled_ingredients {
            let split_range: Vec<&str> = line.split("-").collect();
            if split_range.len() != 2 {
                process::exit(1);
            }
            let start_range = split_range[0].parse::<u64>().unwrap();
            let end_range = split_range[1].parse::<u64>().unwrap();
            fresh_ingredient_ranges.push((start_range, end_range));

        } else {
            let ingredient_id: u64 = line.parse::<u64>().unwrap();
            for (start_range, end_range) in fresh_ingredient_ranges.iter() {
                if *start_range <= ingredient_id && ingredient_id <= *end_range {
                    num_fresh_ingredients += 1;
                    break;
                }
            }
        }
    }

    println!("{}", num_fresh_ingredients);
}
