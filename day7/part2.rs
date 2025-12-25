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

    let result = count_timelines(&start, &splitters, max_x, &mut HashMap::new());
    println!("{}", result);
}

fn count_timelines(
    current_position: &Position,
    splitters: &HashMap<Position, bool>,
    max_x: u32,
    cache: &mut HashMap<Position, u64>,
) -> u64 {
    let next_x = current_position.x + 1;

    if next_x == max_x {
        return 1;
    }

    if cache.contains_key(current_position) {
        return cache[current_position];
    }

    if splitters.contains_key(&Position{x: next_x, y: current_position.y}) {
        let left_pos = Position{x: next_x, y: current_position.y - 1};
        let left_count = count_timelines(
            &left_pos,
            splitters,
            max_x,
            cache,
        );
        cache.insert(left_pos, left_count);

        let right_pos = Position{x: next_x, y: current_position.y + 1};
        let right_count = count_timelines(
            &right_pos,
            splitters,
            max_x,
            cache,
        );
        cache.insert(right_pos, right_count);

        return left_count + right_count;
    }

    let pos = Position{x: next_x, y: current_position.y};
    let count = count_timelines(
        &pos,
        splitters,
        max_x,
        cache,
    );
    cache.insert(pos, count);
    return count;
}
