use std::fs;
use std::cmp;
use std::process;
use std::collections::HashMap;

fn main() {
    let mut fresh_ingredient_ranges: Vec<(u64, u64)> = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        if line == "" {
            break;
        } else {
            let split_range: Vec<&str> = line.split("-").collect();
            if split_range.len() != 2 {
                process::exit(1);
            }
            let start_range = split_range[0].parse::<u64>().unwrap();
            let end_range = split_range[1].parse::<u64>().unwrap();
            fresh_ingredient_ranges.push((start_range, end_range));
        }
    }

    // Merge ranges together.
    fresh_ingredient_ranges.sort();
    let mut new_ranges: Vec<(u64, u64)> = merge_ranges(&fresh_ingredient_ranges);
    while new_ranges.len() != fresh_ingredient_ranges.len() {
        fresh_ingredient_ranges = new_ranges;
        new_ranges = merge_ranges(&fresh_ingredient_ranges);
    }

    // Compute the subtractions.
    let mut num_fresh_ingredient_ids: u64 = 0;
    for (start_range, end_range) in new_ranges {
        num_fresh_ingredient_ids += end_range - start_range + 1;
    }

    println!("{}", num_fresh_ingredient_ids);
}

fn merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_ingredient_ranges: Vec<(u64, u64)> = Vec::new();
    let mut seen_indices : HashMap<usize, bool> = HashMap::new();


    for i in 0..ranges.len() {
        let (start_range, end_range) = ranges[i];

        if seen_indices.contains_key(&i) {
            continue;
        }
        for j in 0..ranges.len() {
            let (start_range_2, end_range_2) = ranges[j];
            if i == j {
                continue;
            }
            if seen_indices.contains_key(&j) {
                continue;
            }

            if start_range <= end_range_2 && start_range_2 <= end_range {
                // Overlap, merge both.
                new_ingredient_ranges.push(
                    (cmp::min(start_range, start_range_2), cmp::max(end_range, end_range_2))
                );
                seen_indices.insert(i, true);
                seen_indices.insert(j, true);
            }
        }
    }

    for i in 0..ranges.len() {
        if !seen_indices.contains_key(&i) {
            let (start_range, end_range) = ranges[i];
            new_ingredient_ranges.push((start_range, end_range));
        }
    }

    return new_ingredient_ranges;
}
