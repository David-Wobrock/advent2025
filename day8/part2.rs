use std::fs;
use std::process;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct Position{x: i64, y: i64, z: i64}

fn main() {
    let mut boxes: Vec<Position> = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let split_range: Vec<&str> = line.split(",").collect();
        if split_range.len() != 3 {
            process::exit(1);
        }
        let x = split_range[0].parse::<i64>().unwrap();
        let y = split_range[1].parse::<i64>().unwrap();
        let z = split_range[2].parse::<i64>().unwrap();
        boxes.push(Position{x, y, z});
    }

    let mut all_distances: Vec<(Position, Position, f64)> = Vec::new();

    for (i, pos) in boxes.iter().enumerate() {
        for other_pos in boxes[i+1..].iter() {
            let distance = euclidean_distance(pos, other_pos);
            all_distances.push((pos.clone(), other_pos.clone(), distance));
        }
    }

    all_distances.sort_by(|(_, _, d1), (_, _, d2)| f64::total_cmp(d1, d2));

    let mut found_boxes: HashMap<Position, usize> = HashMap::new();
    let mut circuits: Vec<HashMap<Position, bool>> = Vec::new();
    let mut i = 0;
    while size_largest_circuit(&circuits) < boxes.len() {
        let (pos, other_pos, _) = all_distances.get(i).unwrap();
        i += 1;

        if !found_boxes.contains_key(pos) && !found_boxes.contains_key(other_pos) {
            let mut new_map: HashMap<Position, bool> = HashMap::new();
            new_map.insert(pos.clone(), true);
            new_map.insert(other_pos.clone(), true);
            circuits.push(new_map);
            found_boxes.insert(pos.clone(), circuits.len() - 1);
            found_boxes.insert(other_pos.clone(), circuits.len() - 1);
        } else if found_boxes.contains_key(pos) && !found_boxes.contains_key(other_pos) {
            let circuit_idx = found_boxes[pos];
            circuits[circuit_idx].insert(other_pos.clone(), true);
            found_boxes.insert(other_pos.clone(), circuit_idx);
        } else if found_boxes.contains_key(other_pos) && !found_boxes.contains_key(pos) {
            let circuit_idx = found_boxes[other_pos];
            circuits[circuit_idx].insert(pos.clone(), true);
            found_boxes.insert(pos.clone(), circuit_idx);
        } else {
            let circuit_idx = found_boxes[pos];
            let circuit_idx_other = found_boxes[other_pos];
            if circuit_idx != circuit_idx_other {
                let c = circuits[circuit_idx_other].clone();
                circuits[circuit_idx].extend(c);
                for p in circuits[circuit_idx_other].keys() {
                    found_boxes.insert(p.clone(), circuit_idx);
                }
                circuits[circuit_idx_other] = HashMap::new();
            }
        }

    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    let (pos, other_pos, _) = all_distances.get(i-1).unwrap();
    println!("{}", pos.x * other_pos.x);
}

fn euclidean_distance(pos1: &Position, pos2: &Position) -> f64 {
    return f64::sqrt(
        (i64::pow(pos1.x - pos2.x, 2) +
        i64::pow(pos1.y - pos2.y, 2) +
        i64::pow(pos1.z - pos2.z, 2)) as f64
    )
}

fn size_largest_circuit(m: &Vec<HashMap<Position, bool>>) -> usize {
    let mut largest_map = 0;
    for c in m {
        if c.len() > largest_map {
            largest_map = c.len();
        }
    }
    return largest_map;
}