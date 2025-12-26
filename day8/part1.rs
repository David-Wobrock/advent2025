use std::fs;
use std::process;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct Position{x: i64, y: i64, z: i64}

fn main() {
    let max_connections = 1000;

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
    let mut used_connections = 0;
    let mut i = 0;
    while used_connections < max_connections {
        let (pos, other_pos, _) = all_distances.get(i).unwrap();
        i += 1;

        if !found_boxes.contains_key(pos) && !found_boxes.contains_key(other_pos) {
            let mut new_map: HashMap<Position, bool> = HashMap::new();
            new_map.insert(pos.clone(), true);
            new_map.insert(other_pos.clone(), true);
            circuits.push(new_map);
            found_boxes.insert(pos.clone(), circuits.len() - 1);
            found_boxes.insert(other_pos.clone(), circuits.len() - 1);
            used_connections += 1;
        } else if found_boxes.contains_key(pos) && !found_boxes.contains_key(other_pos) {
            let circuit_idx = found_boxes[pos];
            circuits[circuit_idx].insert(other_pos.clone(), true);
            found_boxes.insert(other_pos.clone(), circuit_idx);
            used_connections += 1;
        } else if found_boxes.contains_key(other_pos) && !found_boxes.contains_key(pos) {
            let circuit_idx = found_boxes[other_pos];
            circuits[circuit_idx].insert(pos.clone(), true);
            found_boxes.insert(pos.clone(), circuit_idx);
            used_connections += 1;
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
            used_connections += 1;
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("{}", circuits[0].len() * circuits[1].len() * circuits[2].len());
}

fn euclidean_distance(pos1: &Position, pos2: &Position) -> f64 {
    return f64::sqrt(
        (i64::pow(pos1.x - pos2.x, 2) +
        i64::pow(pos1.y - pos2.y, 2) +
        i64::pow(pos1.z - pos2.z, 2)) as f64
    )
}
