use std::fs;

fn main() {
    let mut raw_input: Vec<Vec<char>> = Vec::new();
    let mut current_line: Vec<char>;
    for line in fs::read_to_string("input").unwrap().lines() {
        current_line = Vec::new();
        for c in line.chars() {
            current_line.push(c);
        }
        raw_input.push(current_line);
    }

    let num_rows = raw_input.len();

    let mut grand_total: u64 = 0;
    let mut skip_next = false;
    // For each column.
    let mut operation_numbers: Vec<u64> = Vec::new();
    for col_idx in (0..raw_input[0].len()).rev() {
        if skip_next {
            skip_next = false;
            continue;
        }
        let mut current_num: Vec<char> = Vec::new();
        for row_idx in 0..num_rows-1 {
            current_num.push(raw_input[row_idx][col_idx]);
        }

        let full_number = current_num.iter().collect::<String>().trim().parse::<u64>().unwrap();
        operation_numbers.push(full_number);

        match raw_input[num_rows-1][col_idx] {
            '+' => {
                grand_total += operation_numbers.iter().sum::<u64>();
                operation_numbers = Vec::new();
                skip_next = true;
            },
            '*' => {
                grand_total += operation_numbers.iter().copied().reduce(|a, b| a * b).unwrap();
                operation_numbers = Vec::new();
                skip_next = true;
            },
            _ => {},
        }
    }

    println!("{}", grand_total);
}
