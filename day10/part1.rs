use std::fs;
use std::process;

fn main() {
    let mut targets: Vec<u16> = Vec::new();
    let mut buttons: Vec<Vec<u16>> = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let split_range: Vec<&str> = line.split(" ").collect();

        let target = split_range[0];
        let (target_int, num_bits) = target_to_int(target);
        targets.push(target_int);

        let mut row_buttons: Vec<u16> = Vec::new();
        for i in 1..split_range.len() - 1 {
            let mask = button_to_mask(split_range[i], num_bits);
            row_buttons.push(mask);
        }
        buttons.push(row_buttons);

        // Do nothing with split_range[split_range.len()].
    }

    let mut total_pressed_buttons: u32 = 0;
    // For each line, search.
    for i in 0..targets.len() {
        let target: &u16 = &targets[i];
        let available_buttons: &Vec<u16> = &buttons[i];

        let max_levels = 10; // heuristic
        let mut found = false;
        for j in 1..max_levels {
            let min_buttons_to_press = compute_min_buttons_to_press_dfs(
                target, available_buttons, vec![0; available_buttons.len()], 0, j,
            );
            if min_buttons_to_press > 0 {
                total_pressed_buttons += min_buttons_to_press;
                found = true;
                break;
            }
        }
        if !found {
            println!("Didn't find any path in {} levels", max_levels);
            process::exit(1);
        }
        // Approach too long with BFS.
        //let min_buttons_to_press = compute_min_buttons_to_press_bfs(
        //    target, available_buttons, vec![0; available_buttons.len()], 0,
        //);
        //total_pressed_buttons += min_buttons_to_press;
    }

    println!("{}", total_pressed_buttons);
}

fn target_to_int(s: &str) -> (u16, usize) {
    let l = s.len();
    let content = &s[1..l-1];
    let mut bits: Vec<u16> = Vec::new();
    for c in content.chars() {
        match c {
            '.' => bits.push(0),
            '#' => bits.push(1),
            _ => {},
        }
    }

    let mut result: u16 = 0;
    bits.iter().for_each(|&bit| {
        result <<= 1;
        result ^= bit;
    });
    return (result, bits.len());
}

fn button_to_mask(s: &str, num_bits: usize) -> u16 {
    let l = s.len();
    let content: &str = &s[1..l-1];

    let mut results: u16 = 0;
    for c in content.split(",").collect::<Vec<_>>() {
        let cc: &str = c;
        let d: usize = cc.parse::<usize>().unwrap();
        let shift_count = num_bits - d - 1;
        results |= 1 << shift_count;
    }
    return results;
}

/*
fn compute_min_buttons_to_press_bfs(
    target: &u16,
    available_buttons: &Vec<u16>,
    current_values: Vec<u16>,
    current_level: u32,
) -> u32 {
    let mut next_buttons: Vec<u16> = Vec::new();
    let mut next_values: Vec<u16> = Vec::new();
    for (i, button) in available_buttons.iter().enumerate() {
        let new_value = current_values[i] ^ button;
        if *target == new_value {
            return current_level + 1;
        }

        // Add next level edges.
        for b in available_buttons {
            if b != button { // no need to undo itself
                next_buttons.push(*b);
                next_values.push(new_value);
            }
        }
    }
    return compute_min_buttons_to_press_bfs(target, &next_buttons, next_values, current_level + 1);
}
 */

fn compute_min_buttons_to_press_dfs(
    target: &u16,
    available_buttons: &Vec<u16>,
    current_values: Vec<u16>,
    current_level: u32,
    max_level: u32,
) -> u32 {
    if current_level >= max_level {
        // Stop if reached max depth.
        return 0;
    }
    for (i, button) in available_buttons.iter().enumerate() {
        let new_value = current_values[i] ^ button;
        if *target == new_value {
            return current_level + 1;
        }

        let res = compute_min_buttons_to_press_dfs(
            target,
            available_buttons,
            vec![new_value; available_buttons.len()],
            current_level + 1,
            max_level,
        );
        if res != 0 {
            return res;
        }
    }
    return 0;
}
