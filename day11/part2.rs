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
    println!("{:?}", dfs_num_paths(&connections, "svr", &mut HashMap::new()).0);
}

fn dfs_num_paths(
    connections: &HashMap<String, Vec<String>>,
    current_edge: &str,
    // Value tuple is: (paths with both, paths with only dac, paths with only fft, unknown paths)
    cache: &mut HashMap<String, (u64, u64, u64, u64)>,
) -> (u64, u64, u64, u64) {
    if current_edge == "out" {
        return (0, 0, 0, 1);
    }

    if cache.contains_key(current_edge) {
        return cache[current_edge];
    }

    let mut final_paths_both = 0;
    let mut final_paths_dac = 0;
    let mut final_paths_fft = 0;
    let mut final_paths_unknown = 0;
    for dest in &connections[current_edge] {
        let (paths_both, paths_dac, paths_fft, paths_unknown) = dfs_num_paths(connections, &dest, cache);

        final_paths_both += paths_both;
        final_paths_dac += paths_dac;
        final_paths_fft += paths_fft;
        final_paths_unknown += paths_unknown;
    }

    if current_edge == "dac" {
        final_paths_both += final_paths_fft;
        final_paths_fft = 0;
        final_paths_dac += final_paths_unknown;
        final_paths_unknown = 0;
    } else if current_edge == "fft" {
        final_paths_both += final_paths_dac;
        final_paths_dac = 0;
        final_paths_fft += final_paths_unknown;
        final_paths_unknown = 0;
    }

    cache.insert(current_edge.to_string(), (final_paths_both, final_paths_dac, final_paths_fft, final_paths_unknown));
    (final_paths_both, final_paths_dac, final_paths_fft, final_paths_unknown)
}