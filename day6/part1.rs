use std::fs;
use std::process;

fn main() {
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut symbols: Vec<String> = Vec::new();
    for (i, line) in fs::read_to_string("input").unwrap().lines().enumerate() {
        for (j, l) in line.split_whitespace().enumerate() {
            match l.parse::<u32>() {
                Ok(_) => if i == 0 {
                    problems.push(Vec::from([l.parse::<u64>().unwrap()]))
                } else {
                    problems[j].push(l.parse::<u64>().unwrap())
                },
                Err(_) => symbols.push(l.to_string()),
            }
        }
    }

    let mut grand_total: u64 = 0;
    for i in 0..problems.len() {
        match symbols[i].as_ref() {
            "+" => grand_total += problems[i].iter().sum::<u64>(),
            "*" => grand_total += problems[i].iter().copied().reduce(|a, b| a*b).unwrap(),
            &_ => process::exit(1),
        }
    }

    println!("{}", grand_total);
}
