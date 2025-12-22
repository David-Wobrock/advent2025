use std::fs;

fn main() {
    let mut sum: u64 = 0;

    for line in fs::read_to_string("input").unwrap().lines() {
        let (left_value, left_value_idx) = find_highest_value(&line[..line.len() - 1]);
        let (right_value, _) = find_highest_value(&line[left_value_idx+1..]);

        let result = format!("{}{}", left_value, right_value);
        sum += result.parse::<u64>().unwrap();
    }

    println!("{}", sum);
}

fn find_highest_value(line: &str) -> (u32, usize) {
    let mut highest_value: u32 = 0;
    let mut highest_value_idx: usize = 0;

    for (i, c) in line.chars().enumerate() {
        let d = c.to_digit(10).unwrap();
        if d > highest_value {
            highest_value = d;
            highest_value_idx = i;
        }
        if d == 9 {
            break;
        }
    }

    return (highest_value, highest_value_idx);
}