use std::fs;
use std::collections::HashSet;

const INPUT_FILE: &str = "inputs/day-06.txt";
const BUF_SIZE: usize = 14; // 4;

fn find(raw: &str, size: usize) -> Option<usize> {
    for i in 0..(raw.len()-size) {
        let chunk = &raw[i..i+size];
        let chars: HashSet<_> = chunk.chars().collect();
        if chars.len() == size {
            return Some(i + size);
        }
    }
    None
}

fn main() {
    let raw = fs::read_to_string(INPUT_FILE).unwrap();
    if let Some(i) = find(raw.as_str(), BUF_SIZE) {
        println!("Found a match at index {}", i);
    } else {
        println!("No match found!");
    }
}
