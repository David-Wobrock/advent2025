use std::fs;

fn main() {
    let num_size = 12;
    let mut sum: u64 = 0;

    for line in fs::read_to_string("input").unwrap().lines() {
        let mut res: Vec<String> = Vec::new();
        let mut highest_val_idx: usize = 0;

        for i in (0..num_size).rev() {

            let (highest_val, new_highest_val_idx) = find_highest_value(
                &line[highest_val_idx..line.len()-i]
            );
            res.push(highest_val.to_string());
            highest_val_idx += new_highest_val_idx+1;
        }

        let result = res.join("");
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