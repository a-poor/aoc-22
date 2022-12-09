use std::fs;
use std::collections::HashSet;

const INPUT_PATH: &str = "inputs/day-03.txt";


fn priority(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        c as i32 - 'a' as i32 + 1
    } else if c >= 'A' && c <= 'Z' {
        c as i32 - 'A' as i32 + 27
    } else {
        panic!("can't prioritize unknown character '{}'", c);
    }
}


fn into_threes(lines: Vec<&str>) -> Vec<Vec<&str>> {
    // Initialize a place to store the data...
    let mut res: Vec<Vec<&str>> = Vec::new();
    let mut group: Vec<&str> = Vec::new();
    
    // Iterate through the lines...
    for line in lines {
        // Add the line to the current group
        group.push(line);

        // If it reached the right length, add it and reset the group
        if group.len() == 3 {
            res.push(group);
            group = Vec::new();
        }
    }

    // Return results!
    res
}

fn group_intersect(groups: Vec<Vec<&str>>) -> Vec<char> {
    groups
        .into_iter()
        .enumerate()
        .map(|(i, group)| {
            let intersect: Vec<char> = group
                .into_iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|a, b| {
                    let int = a.intersection(&b);
                    int
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|c| *c)
                        .collect::<HashSet<_>>()
                })
                .expect("group intersection is empty")
                .into_iter()
                .collect()
                ;
            
            if intersect.len() == 0 {
                panic!("zero length intersection for group {}", i);
            }
            if intersect.len() > 1 {
                panic!("intersection greater than 1 for group: {}", i);
            }

            let ci = intersect[0];
            ci
        })
        .collect()
}


fn main() {
    let raw = fs::read_to_string(INPUT_PATH)
        .expect("failed to read in file");

    let raw = raw.trim();
    let lines: Vec<&str> = raw.split("\n").collect();

    let groups = into_threes(lines);
    let total = group_intersect(groups)
        .into_iter()
        .map(|c| priority(c))
        .reduce(|a, b| a + b)
        .expect("no lines!");

    println!("Result = {}", total);
}

