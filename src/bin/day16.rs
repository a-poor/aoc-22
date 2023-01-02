#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
use std::collections::{HashMap, VecDeque};
use regex::Regex;


#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

fn parse_line(line: &str) -> Result<Valve, String> {
    // Form the regex...
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? (.+)")
        .expect("failed to compile regex");

    // Parse the line...
    let caps = re.captures(line)
        .ok_or(format!("failed to parse line: {}", line))?;

    // Extract the parts...
    let name = caps
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    let flow_rate = caps
        .get(2)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .map_err(|e| format!("failed to parse flow rate: {}", e))?;
    let tunnels = caps
        .get(3)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Success!
    Ok(Valve { name, flow_rate, tunnels })
}


struct State {
    valves: HashMap<String, Valve>,
    queue: VecDeque<String>,
    visited: HashMap<String, u32>,
}

fn main() -> Result<(), String> {
    // Read the input data...
    let input = aoc_22::util::load_input(16, true)?;



    // Success!
    Ok(())
}
