use std::fs;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
struct TupleStruct{x: i32, y: i32}

fn main() {
    let mut num_rolls_accessible: u64 = 0;

    let mut rolls: HashMap<TupleStruct, bool> = HashMap::new();
    for (x, line) in fs::read_to_string("input").unwrap().lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '@' {
                rolls.insert(TupleStruct{x: x as i32, y: y as i32}, true);
            }
        }
    }

    let mut num_rolls = rolls.len();
    while true {
        for roll_pos in rolls.keys() {
            let x = roll_pos.x;
            let y = roll_pos.y;
            let mut num_neighbors = 0;
            // Above
            for i in -1..2 {
                if rolls.contains_key(&TupleStruct { x: x - 1, y: y + i }) {
                    num_neighbors += 1;
                }
            }
            // Below
            for i in -1..2 {
                if rolls.contains_key(&TupleStruct { x: x + 1, y: y + i }) {
                    num_neighbors += 1;
                }
            }
            // Left & right
            if rolls.contains_key(&TupleStruct { x: x, y: y - 1 }) {
                num_neighbors += 1;
            }
            if rolls.contains_key(&TupleStruct { x: x, y: y + 1 }) {
                num_neighbors += 1;
            }

            if num_neighbors < 4 {
                num_rolls_accessible += 1;
                rolls.remove(&TupleStruct{x: x, y: y});
                break;
            }
        }

        let new_num_rolls = rolls.len();
        if new_num_rolls == num_rolls {
            break;
        }
        num_rolls = new_num_rolls;
    }

    println!("{}", num_rolls_accessible);
}
