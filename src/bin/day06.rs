use std::fs;
use std::collections::HashSet;

const INPUT_FILE: &str = "inputs/day-06.txt";
const BUF_SIZE: usize = 4;

fn find(raw: &str) -> Option<usize> {
    for i in 0..(raw.len()-BUF_SIZE) {
        let chunk = &raw[i..i+BUF_SIZE];
        let chars: HashSet<_> = chunk.chars().collect();
        if chars.len() == BUF_SIZE {
            return Some(i + BUF_SIZE);
        }
    }
    None
}

fn main() {
    let raw = fs::read_to_string(INPUT_FILE).unwrap();
    if let Some(i) = find(raw.as_str()) {
        println!("Found a match at index {}", i);
    } else {
        println!("No match found!");
    }
}
