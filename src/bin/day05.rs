use std::fs;
use std::collections::VecDeque;


const INPUT_FILE: &str = "inputs/day-05.txt";

type State<T> = Vec<VecDeque<T>>;

fn split_line_by_three(line: &str) -> Vec<&str> {
    // Validate the line length
    if line.len() % 3 != 0 {
        panic!("line length isn't divisible by 3: {}", line.len());
    }

    // How many chunks are there...
    let n = line.len() / 3;

    // Create a place to store the result...
    let mut res: Vec<_> = Vec::new();

    // Iterate throuth....
    for i in 1..n {
        // Calculate the indices...
        let start = 3 * i;
        let stop = 3 * (i+1);

        // Get the chunk...
        let seg = &line[start..stop];

        // Add it to the vector...
        res.push(seg);
    }

    // Return result!
    res
}

fn main() {
    let raw = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";



}
