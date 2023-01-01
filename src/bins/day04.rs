use std::fs;

const INPUT_FILE: &str = "inputs/day-04.txt";


#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Self {start, end}
    }
}

struct Pair(Range, Range);


fn overlaps(p: Pair) -> bool {
    let Pair(a, b) = p;
    
    if a.start <= b.start && a.end >= b.start {
        true
    } else if b.start <= a.start && b.end >= a.start {
        true
    } else {
        false
    }
}

fn split_range(range: &str) -> Range {
    let split: Vec<&str> = range.split("-").collect();
    if split.len() != 2 {
        panic!("unknown range value {}", range);
    }
    
    let a = split[0];
    let b = split[1];

    let a = a.to_string().parse().expect("couldn't parse range number as int");
    let b = b.to_string().parse().expect("couldn't parse range number as int");

    Range::new(a, b)
}

fn split_line(line: &str) -> Pair {
    let split: Vec<&str> = line.split(",").collect();
    if split.len() != 2 {
        panic!("unknown line length after split {}", line);
    }

    let a = split[0];
    let b = split[1];

    let a = split_range(a);
    let b = split_range(b);

    Pair(a, b)
}

fn main() {
    let raw = fs::read_to_string(INPUT_FILE)
        .expect("failed to read input file");

    let lines = raw
        .trim()
        .split("\n")
        .map(split_line)
        .map(|p| {
            if overlaps(p) {
                1
            } else {
                0
            }
        })
        .reduce(|a, b| a + b)
        .expect("no lines to parse")
        ;

    println!("# of subsuming pairs: {}", lines);

}

