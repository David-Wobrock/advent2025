use std::fs;
use std::process;

fn main() {
    let mut num_clicks_over_zero = 0;
    let mut current_position = 50;
    for line in fs::read_to_string("input").unwrap().lines() {
        let mut line_chars = line.chars();
        let num_rotations = line[1..line.len()].parse::<i32>().unwrap();
        match line_chars.nth(0).unwrap() {
            'L' => {
                if current_position == 0 {
                    num_clicks_over_zero += (current_position + num_rotations) / 100;
                } else {
                    num_clicks_over_zero += (current_position - num_rotations - 100).abs() / 100;
                }
                current_position = (current_position - num_rotations).rem_euclid(100);
            },
            'R' => {
                num_clicks_over_zero += (current_position + num_rotations) / 100;
                current_position = (current_position + num_rotations).rem_euclid(100);
            },
            _ => process::exit(1)
        }
    }
    println!("{}", num_clicks_over_zero)
}
