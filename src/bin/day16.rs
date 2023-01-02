#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
use std::collections::{HashMap, VecDeque};
use regex::Regex;

struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

fn dfs(valves: &HashMap<String, Valve>, max_time: i32, elapsed_time: i32, pressure: i32, visited: &mut Vec<String>) -> i32 {
    // Return the current pressure if we have exceeded the maximum time
    if elapsed_time > max_time {
        return pressure;
    }

    // Keep track of the maximum pressure achieved
    let mut max_pressure = pressure;

    // Explore each unvisited tunnel from the current valve
    for tunnel in &valves[&visited[visited.len() - 1]].tunnels {
        if !visited.contains(tunnel) {
            // Open the valve and add it to the visited list
            visited.push(tunnel.to_string());
            let new_pressure = pressure + valves[tunnel].flow_rate;

            // Recursively search from the new valve
            let p = dfs(valves, max_time, elapsed_time + 2, new_pressure, visited);
            max_pressure = i32::max(max_pressure, p);

            // Backtrack by closing the valve and removing it from the visited list
            visited.pop();
        }
    }

    max_pressure
}

fn main() {
    // Read the input file and parse the valves and tunnels
    let input = fs::read_to_string("inputs/day-16.txt").unwrap();
    let valve_regex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let mut valves = HashMap::new();
    for line in input.lines() {
        let caps = valve_regex.captures(line).unwrap();
        let name = caps[1].to_string();
        let flow_rate = caps[2].parse::<i32>().unwrap();
        let tunnels: Vec<String> = caps[3].split(", ").map(|s| s.to_string()).collect();
        valves.insert(name, Valve { flow_rate, tunnels });
    }

    // Start the search from valve AA
    let mut visited = vec!["AA".to_string()];
    let max_pressure = dfs(&valves, 30, 0, 0, &mut visited);

    println!("Maximum pressure release: {}", max_pressure);
}

