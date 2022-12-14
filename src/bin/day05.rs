use std::fs;
use regex::Regex;
use std::collections::VecDeque;


const INPUT_FILE: &str = "inputs/day-05.txt";

type Stacks = Vec<VecDeque<char>>;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_box_row(line: &str) -> Vec<Option<char>> {
    // Create a vec to store the results...
    let mut res = Vec::new();

    // Chunks of 3 separated by spaces
    let n = (line.len() - 3) / 4 + 1;

    // Iterate through the lines...
    for i in 0..n {
        let start = 3*i + res.len();
        let end = start + 3;
        
        let c = &line[start..end];
        res.push(c);
    }

    // Convert the chunks into 
    res
        .into_iter()
        .map(|chunk| {
            let chunk = chunk
                .chars()
                .nth(1)
                .expect("couldn't get the 2nd character");

            if chunk == ' ' {
                None
            } else {
                Some(chunk)
            }
        })
        .collect()
}

fn pivot_boxes(boxes: Vec<Vec<Option<char>>>) -> Stacks {
    let n = boxes[0].len();
    let mut res: Vec<VecDeque<char>> = (0..n)
        .map(|_| VecDeque::new())
        .collect();

    for r in boxes {
        for (j, c) in r.into_iter().enumerate() {
            if let Some(c) = c {
                res[j].push_back(c);
            }
        }
    }
    res
}

fn parse_move(line: &str) -> Move {
    let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let caps = re.captures(line).unwrap();

    let nmove = caps.get(1).unwrap().as_str();
    let nfrom = caps.get(2).unwrap().as_str();
    let nto = caps.get(3).unwrap().as_str();

    Move {
        count: nmove.parse().unwrap(),
        from: nfrom.parse::<usize>().unwrap() - 1,
        to: nto.parse::<usize>().unwrap() - 1,
    }
}

fn main() {
    let raw = fs::read_to_string(INPUT_FILE)
        .expect("failed to read input file");

    let split: Vec<&str> = raw.split("\n\n").collect();
    let raw_boxes = split[0];
    let raw_moves = split[1];

    let grid: Vec<_> = raw_boxes
        .split("\n")
        .map(|line| parse_box_row(line))
        .collect();
    let mut boxes = pivot_boxes(grid);

    let moves: Vec<_> = raw_moves
        .split("\n")
        .map(|line| parse_move(line))
        .collect();

    for m in moves {
        let mut from_stack = boxes
            .get(m.from)
            .expect(format!("failed to get 'from' stack number {}", m.from).as_str())
            .clone();
        let mut to_stack = boxes
            .get(m.to)
            .expect(format!("failed to get 'to' stack number {}", m.to).as_str())
            .clone();

        // Use a buffer, in case the boxes need to be moved in place...
        let mut buf = Vec::new();
        for i in 0..m.count {
            let c = from_stack
                .pop_front()
                .expect(
                    format!(
                        "failed to get {}th value in from-stack {}", 
                        i, m.from
                    )
                    .as_str()
                );
            buf.push(c);
        }
        
        // // Part 1: Use the following (move boxes one at a time)...
        // for c in buf {
        //     to_stack.push_front(c);
        // }

        // Part 2: Use the following (move boxes "all at once")...
        for c in buf.into_iter().rev() {
            to_stack.push_front(c);
        }

        // Re-store the stacks...
        boxes[m.from] = from_stack;
        boxes[m.to] = to_stack;
    }

    print!("Top boxes: ");
    for col in boxes {
        print!("{}", col[0]);
    }
    println!("");

}
