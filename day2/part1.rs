use std::fs;
use std::process;

fn main() {
    let mut ranges: Vec<(u64, u64)> = vec![];

    for line in fs::read_to_string("input").unwrap().lines() {
        let raw_ranges = line.split(",");
        for raw_range in raw_ranges {
            let split_range: Vec<&str> = raw_range.split("-").collect();
            if split_range.len() != 2 {
                process::exit(1);
            }
            let start_range = split_range[0].parse::<u64>().unwrap();
            let end_range = split_range[1].parse::<u64>().unwrap();

            ranges.push((start_range, end_range));
        }
    }

    let mut sum: u64 = 0;

    for (start, end) in ranges {
        for i in start..end {
            let s = i.to_string();
            let l = s.len();

            if (l % 2) == 0 && s[0..l/2] == s[l/2..l] {
                sum += i;
            }
        }
    }

    println!("{}", sum);
}
