use std::fs;
use std::collections::HashMap;

fn main() {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let split_range: Vec<&str> = line.split(":").collect();

        let source = split_range[0];
        let destinations: Vec<String> = split_range[1].trim().split(" ").map(|v: &str| v.to_string()).collect();
        connections.insert(source.to_string(), destinations);
    }
    println!("{:?}", dfs_num_paths(&connections, "you"));
}

fn dfs_num_paths(connections: &HashMap<String, Vec<String>>, current_edge: &str) -> i32 {
    if current_edge == "out" {
        return 1;
    }

    let mut paths = 0;
    for dest in &connections[current_edge] {
        paths += dfs_num_paths(connections, &dest);
    }
    paths
}