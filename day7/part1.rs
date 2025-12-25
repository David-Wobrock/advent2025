use std::fs;
use std::process;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Position{x: u32, y: u32}

fn main() {
    let mut start: Position = Position{x: 0, y: 0};
    let mut splitters: HashMap<Position, bool> = HashMap::new();
    let mut max_x: u32 = 0;
    for (i, line) in fs::read_to_string("input").unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Position{x: i as u32, y: j as u32};
            } else if c == '^' {
                splitters.insert(Position{x: i as u32, y: j as u32}, true);
            }
        }
        max_x = i as u32;
    }

    if start.y == 0 {
        process::exit(1);
    }

    let mut num_splitters_encounters: u64 = 0;
    let mut current_y_positions: Vec<u32> = [start.y].to_vec();
    let mut new_y_positions: HashMap<u32, bool>;
    // From top to bottom
    for x in 1..max_x {
        new_y_positions = HashMap::new();

        for y in current_y_positions.iter() {
            if splitters.contains_key(&Position{x: x as u32, y: *y as u32}) {
                // Encountered a splitter
                num_splitters_encounters += 1;
                new_y_positions.insert(y-1, true);
                new_y_positions.insert(y+1, true);
            } else {
                new_y_positions.insert(*y, true);
            }
        }
        current_y_positions = new_y_positions.keys().cloned().collect();
    }

    println!("{}", num_splitters_encounters);
}
