use std::fs;
use std::process;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct Position{x: i64, y: i64}

fn main() {
    let mut red_tiles: Vec<Position> = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let split_range: Vec<&str> = line.split(",").collect();
        if split_range.len() != 2 {
            process::exit(1);
        }
        let x = split_range[0].parse::<i64>().unwrap();
        let y = split_range[1].parse::<i64>().unwrap();
        red_tiles.push(Position{x, y});
    }

    let mut largest_area: i64 = 0;
    for (i, tile_pos) in red_tiles.iter().enumerate() {
        for other_tile_pos in red_tiles[i+1..].iter() {
            let area = compute_area(tile_pos, other_tile_pos);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("{}", largest_area);
}

fn compute_area(pos1: &Position, pos2: &Position) -> i64 {
    return ((pos1.x - pos2.x).abs() + 1) * ((pos1.y - pos2.y).abs() +1);
}
