use std::fs;

const INPUT_FILE: &str = "inputs/day-04.txt";


#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

struct Pair(Range, Range);

fn a_in_b(r1: Range, r2: Range) -> bool {
    r1.start < r2.start && r1.end < r2.end
}

fn subsumes(p: Pair) -> bool {
    let Pair(a, b) = p;
    a_in_b(a, b) || a_in_b(b, a)
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

    Range { start: a, end: b }
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
    let raw = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let lines = raw
        .trim()
        .split("\n")
        .map(split_line)
        .map(|p| {
            if subsumes(p) {
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
