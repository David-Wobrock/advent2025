use std::fs;
use std::cmp;
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

    let mut edges: Vec<(Position, Position)> = Vec::new();
    for i in 0..red_tiles.len() {
        edges.push((red_tiles[i].clone(), red_tiles[(i+1) % red_tiles.len()].clone()));
    }

    let mut largest_area: i64 = 0;
    for (i, tile_pos) in red_tiles.iter().enumerate() {
        for other_tile_pos in red_tiles[i+1..].iter() {
            let area = compute_area(tile_pos, other_tile_pos);
            if area > largest_area && is_included_in_structure(&red_tiles, tile_pos, other_tile_pos) {
                largest_area = area;
            }
        }
    }

    // 4600181596 is too high.
    // 4508605978 is too high.
    // 4738473600 incorrect.
    // 225807048 is too low.
    println!("{}", largest_area);
}

fn compute_area(pos1: &Position, pos2: &Position) -> i64 {
    return ((pos1.x - pos2.x).abs() + 1) * ((pos1.y - pos2.y).abs() +1);
}

fn is_included_in_structure(
    polygon: &Vec<Position>,
    pos1: &Position,
    pos2: &Position,
) -> bool {
    let min_x = cmp::min(pos1.x, pos2.x) as f64;
    let max_x = cmp::max(pos1.x, pos2.x) as f64;
    let min_y = cmp::min(pos1.y, pos2.y) as f64;
    let max_y = cmp::max(pos1.y, pos2.y) as f64;

    let mid_x = (min_x + max_x) / 2.0;
    let mid_y = (min_y + max_y) / 2.0;

    let test_points = [
        (mid_x, mid_y), // Center
        (mid_x, min_y), // Mid-bottom
        (mid_x, max_y), // Mid-top
        (min_x, mid_y), // Mid-left
        (max_x, mid_y), // Mid-right
    ];
    for (tx, ty) in test_points {
        if !is_inside_f64(tx, ty, polygon) && !is_on_boundary_f64(tx, ty, polygon) {
            return false;
        }
    }

    let rect_corners = [
        (min_x, min_y), (max_x, min_y), (max_x, max_y), (min_x, max_y)
    ];

    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        for j in 0..4 {
            let r1 = rect_corners[j];
            let r2 = rect_corners[(j + 1) % 4];
            if edges_intersect(r1.0, r1.1, r2.0, r2.1, p1.x as f64, p1.y as f64, p2.x as f64, p2.y as f64) {
                return false;
            }
        }
    }

    return true;
}

fn is_inside_f64(x: f64, y: f64, polygon: &[Position]) -> bool {
    let mut inside = false;
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        // Winding/Ray casting logic:
        // Checks if point is within the y-range and to the left of the edge
        if ((p1.y as f64 > y) != (p2.y as f64 > y)) &&
            (x < (p2.x as f64 - p1.x as f64) * (y - p1.y as f64) / (p2.y as f64 - p1.y as f64) + p1.x as f64) {
            inside = !inside;
        }
    }
    return inside;
}

fn is_on_boundary_f64(x: f64, y: f64, polygon: &[Position]) -> bool {
    let epsilon = 1e-9;
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        let (x1, y1, x2, y2) = (p1.x as f64, p1.y as f64, p2.x as f64, p2.y as f64);

        if x >= x1.min(x2) - epsilon && x <= x1.max(x2) + epsilon &&
            y >= y1.min(y2) - epsilon && y <= y1.max(y2) + epsilon {
            let dist = ((y2 - y1) * x - (x2 - x1) * y + x2 * y1 - y2 * x1).abs() /
                ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();
            if dist < epsilon { return true; }
        }
    }
    false
}

fn edges_intersect(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, x4: f64, y4: f64) -> bool {
    fn det(a: f64, b: f64, c: f64, d: f64) -> f64 { a * d - b * c }

    let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if d == 0.0 { return false; }

    let t = det(x1 - x3, x3 - x4, y1 - y3, y3 - y4) / d;
    let u = det(x1 - x3, x1 - x2, y1 - y3, y1 - y2) / d;

    return t > 0.001 && t < 0.999 && u > 0.001 && u < 0.999
}
