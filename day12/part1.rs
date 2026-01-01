use std::fs;
use std::collections::HashMap;

fn main() {
    let (presents, regions) = read_input();

    let mut num_regions_where_presents_fit = 0;
    for region in regions.iter() {
        if region_can_fit_presents(&region, &presents) {
            num_regions_where_presents_fit += 1;
        }
    }
    println!("{}", num_regions_where_presents_fit);
}

fn read_input() -> (Vec<HashMap<(u16, u16), bool>>, Vec<(u16, u16, Vec<u16>)>) {
    let mut presents: Vec<HashMap<(u16, u16), bool>> = Vec::new();
    let mut regions: Vec<(u16, u16, Vec<u16>)> = Vec::new();

    let max_presents = 6;
    let present_input_len = 5;
    let mut current_present: HashMap<(u16, u16), bool> = HashMap::new();
    let mut current_present_num = 0;
    let mut idx_in_present = 0;
    for (i, line) in fs::read_to_string("input").unwrap().lines().enumerate() {
        if i < max_presents * present_input_len {
            if idx_in_present == 0 {
                // First row, skip.
                idx_in_present += 1;
                continue;
            }
            if line == "" {
                // Last element, empty row.
                presents.push(current_present);
                current_present = HashMap::new();
                current_present_num += 1;
                idx_in_present = 0;
            }
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    current_present.insert(
                        (
                            (i - 1 - (current_present_num * present_input_len)) as u16,
                            j as u16,
                        ),
                        true,
                    );
                }
            }
        } else {
            regions.push(line_to_region(line));
        }
    }

    (presents, regions)
}

fn line_to_region(line: &str) -> (u16, u16, Vec<u16>) {
    let split_range: Vec<_> = line.split(':').collect();
    let dimensions: Vec<_> = split_range[0].split('x').collect();
    let length = dimensions[0].parse::<u16>().unwrap();
    let width = dimensions[1].parse::<u16>().unwrap();

    let mut gifts: Vec<u16> = Vec::new();
    for gift in split_range[1].trim().split_whitespace() {
        gifts.push(gift.parse::<u16>().unwrap());
    }

    (length, width, gifts)
}

fn region_can_fit_presents(region: &(u16, u16, Vec<u16>), presents: &Vec<HashMap<(u16, u16), bool>>) -> bool {
    let num_squares_region = region.0 * region.1;

    let mut num_squares_presents: u16 = 0;
    for (i, p) in presents.iter().enumerate() {
        num_squares_presents += p.len() as u16 * region.2[i];
    }

    return num_squares_presents <= num_squares_region;
}