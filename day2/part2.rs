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

            for mul in 2..l+1 {
                if l % mul == 0 {
                    if (l/mul..l).step_by(l/mul).all(|i| {
                        s[0..l/mul] == s[i..i+(l/mul)]
                    }) {
                        sum += i;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", sum);
}
