// use std::fs;
use std::collections::HashSet;

// const INPUT_FILE: &str = "inputs/day-06.txt";
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
    assert_eq!(find("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
    assert_eq!(find("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
    assert_eq!(find("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
    assert_eq!(find("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
    assert_eq!(find("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));


    let raw = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    if let Some(i) = find(raw) {
        println!("Found a match at index {}", i);
    } else {
        println!("No match found!");
    }
}
