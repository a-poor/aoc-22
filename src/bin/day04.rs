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

impl Pair {
    fn new(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> Self {
        Pair(
            Range::new(a_start, a_end),
            Range::new(b_start, b_end),
        )
    }
}

fn a_in_b(r1: Range, r2: Range) -> bool {
    r1.start >= r2.start && r1.end <= r2.end
}

fn subsumes(p: Pair) -> bool {
    let Pair(a, b) = p;
    a_in_b(a, b) || a_in_b(b, a)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_in_b() {
        assert!(!a_in_b(
            Range{start: 2, end: 8}, 
            Range{start: 3, end: 5},
        ));
        assert!(!a_in_b(
            Range{start: 1, end: 3}, 
            Range{start: 2, end: 4},
        ));
        assert!(a_in_b(
            Range{start: 3, end: 5}, 
            Range{start: 1, end: 10},
        ));
    }
    
    #[test]
    fn test_overlaps() {
        assert!(overlaps(Pair::new(
            1, 5,
            2, 6,
        )));
        assert!(overlaps(Pair::new(
            1, 10,
            3, 6,
        )));
        assert!(overlaps(Pair::new(
            3, 5,
            1, 10,
        )));
        assert!(overlaps(Pair::new(
            3, 9,
            2, 6,
        )));
        

        assert!(!overlaps(Pair::new(
            1, 4,
            6, 8,
        )));
        assert!(!overlaps(Pair::new(
            6, 8,
            1, 4,
        )));
    }

}
