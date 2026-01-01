use std::fs;

fn main() {
    let mut target_levels: Vec<Vec<u16>> = Vec::new();
    let mut buttons: Vec<Vec<Vec<u16>>> = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let split_range: Vec<&str> = line.split(" ").collect();

        // Do nothing with split_range[0].
        let target_level = target_to_vec(split_range[split_range.len()-1]);
        target_levels.push(target_level.clone());

        let mut row_buttons: Vec<Vec<u16>> = Vec::new();
        for i in 1..split_range.len() - 1 {
            let button_vec = raw_button_to_vec(split_range[i], target_level.len());
            row_buttons.push(button_vec);
        }
        buttons.push(row_buttons);
    }

    let mut total_pressed_buttons: u32 = 0;
    // For each line, search.
    for i in 0..target_levels.len() {
        let target: &Vec<u16> = &target_levels[i];
        let available_buttons: Vec<Vec<u16>> = buttons[i].clone();

        let lowest_pressed_buttons: u16 = compute_min_buttons_to_press_dfs(target, &available_buttons);
        total_pressed_buttons += lowest_pressed_buttons as u32;
    }

    println!("{}", total_pressed_buttons);
}

fn target_to_vec(s: &str) -> Vec<u16> {
    let l = s.len();
    let content = &s[1..l-1];

    let mut result: Vec<u16> = Vec::new();
    for c in content.split(",").collect::<Vec<_>>() {
        let cc: &str = c;
        let d: u16 = cc.parse::<u16>().unwrap();
        result.push(d);
    }
    return result;
}

fn raw_button_to_vec(s: &str, num_digits: usize) -> Vec<u16> {
    let l = s.len();
    let content: &str = &s[1..l-1];

    let mut result: Vec<u16> = vec![0; num_digits];
    for c in content.split(",").collect::<Vec<_>>() {
        let cc: &str = c;
        let d: usize = cc.parse::<usize>().unwrap();
        result[d] = 1;
    }
    return result;
}

fn compute_min_buttons_to_press_dfs(
    target: &Vec<u16>,
    available_buttons: &Vec<Vec<u16>>,
) -> u16 {
    if target.iter().all(|x| *x == 0) {
        // All elements are 0.
        return 0;
    }

    let (target_value, affected_buttons) = get_target_field_to_compute(target, available_buttons);
    let mut other_buttons = Vec::new();
    for b in available_buttons {
        if !affected_buttons.contains(&b) {
            other_buttons.push(b.clone());
        }
    }

    let mut results: Vec<u16> = Vec::new();
    let all_possible_partitions = possible_partitions(&affected_buttons, target_value, target);
    for partitions in all_possible_partitions {
        let mut new_target: Vec<u16> = target.clone();
        for (i, p) in partitions.iter().enumerate() {
            new_target = sub_vecs_x_times(&new_target, &affected_buttons[i], *p);
        }
        let res =  compute_min_buttons_to_press_dfs(&new_target, &other_buttons);
        if res != u16::max_value() {
            results.push(res + target_value);
        }
    }

    return match results.iter().filter(|x| **x != 0).min() {
        None => u16::max_value(),
        Some(x) => *x,
    };
}

fn mul_vecs(a: &Vec<u16>, x: u16) -> Vec<u16> {
    let mut new: Vec<u16> = Vec::new();
    for aval in a {
        new.push(aval * x);
    }
    return new;
}

fn sub_vecs_x_times(a: &Vec<u16>, b: &Vec<u16>, x: u16) -> Vec<u16> {
    let mut new: Vec<u16> = Vec::new();

    for (aval, bval) in a.iter().zip(b) {
        new.push(aval - (bval * x));
    }
    return new;
}

fn any_value_above_target(current_values: &Vec<u16>, target: &Vec<u16>) -> bool {
    for (current_val, target_val) in current_values.iter().zip(target) {
        if current_val > target_val {
            return true;
        }
    }
    return false;
}

fn get_target_field_to_compute(target: &Vec<u16>, available_buttons: &Vec<Vec<u16>>) -> (u16, Vec<Vec<u16>>) {
    let mut target_value: u16 = 0;
    let mut result_affected_buttons: Vec<Vec<u16>> = Vec::new();

    for i in 0..target.len() {
        if target[i] == 0 {
            // Already completed.
            continue;
        }

        let mut affected_buttons: Vec<Vec<u16>> = Vec::new();
        for b in available_buttons {
            if b[i] == 1 {
                affected_buttons.push(b.clone());
            }
        }

        if affected_buttons.len() < result_affected_buttons.len() || result_affected_buttons.len() == 0 {
            target_value = target[i];
            result_affected_buttons = affected_buttons.clone();
        } else if affected_buttons.len() == result_affected_buttons.len() && target[i] > target_value {
            target_value = target[i];
            result_affected_buttons = affected_buttons.clone();
        }
    }

    return (target_value, result_affected_buttons);
}

fn possible_partitions(affected_buttons: &Vec<Vec<u16>>, value_to_split: u16, target: &Vec<u16>) -> Vec<Vec<u16>> {
    let mut result: Vec<Vec<u16>> = Vec::new();

    if affected_buttons.len() == 1 {
        if any_value_above_target(&mul_vecs(&affected_buttons[0], value_to_split), &target) {
            // This cannot work, return empty array.
            return Vec::new();
        }
        return vec![vec![value_to_split]];
    }

    for value_of_first_button in 0..value_to_split+1 {
        if any_value_above_target(&mul_vecs(&affected_buttons[0], value_of_first_button), &target) {
            // Skip because any value of this button presses of the first button is above the target.
            // We can stop entirely, because all other values will be above.
            break;
        }
        for rest in possible_partitions(
            &affected_buttons[1..].to_vec(),
            value_to_split - value_of_first_button,
            &sub_vecs_x_times(target, &affected_buttons[0], value_of_first_button),
        ) {
            if rest.len() != 0 {
                let mut partition: Vec<u16> = vec![value_of_first_button];
                partition.extend(rest);
                result.push(partition);
            }
        }
    }

    return result;
}