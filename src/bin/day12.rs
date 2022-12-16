use std::fs;
use std::collections::VecDeque;

const INPUT_PATH: &str = "inputs/day-12.txt";

struct StartData {
    grid: Vec<i32>,
    width: i32,
    start: i32,
    end: i32,
}

fn char_to_height(c: char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,

        'S' => char_to_height('a'),
        'E' => char_to_height('z'),

        _ => panic!("invalid character: \"{}\"", c),
    }
}

fn parse_input(path: &str) -> StartData {
    let mut grid = Vec::new();
    let mut start = 0;
    let mut end = 0;

    let raw = fs::read_to_string(path)
        .expect("failed to read input data");
    let lines: Vec<_> = raw
        .lines()
        .collect();
    let width = lines[0]
        .chars()
        .count() as i32;

    for (i, line) in lines.into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let idx = idx_2d_to_1d(i as i32, j as i32, width);
            if c == 'S' {
                start = idx;
            }
            if c == 'E' {
                end = idx;
            }
            grid.push(char_to_height(c));
        }
    }

    StartData { grid, width, start, end }
}

fn check_move(from_height: i32, to_height: i32) -> bool {
    from_height + 1 <= to_height
}

fn idx_1d_to_2d(i: i32, w: i32) -> (i32, i32) {
    (i / w, i % w)
}

fn idx_2d_to_1d(i: i32, j: i32, w: i32) -> i32 {
    i * w + j
}

fn distance(a: i32, b: i32, w: i32) -> i32 {
    // Convert from 1D to 2D "points"...
    let (ax, ay) = idx_1d_to_2d(a, w);
    let (bx, by) = idx_1d_to_2d(b, w);

    // Get the deltas...
    let dx = bx - ax;
    let dy = by - ay;

    // Return the manhattan distance...
    dx.abs() + dy.abs()
}

fn eval_path(path: &Vec<i32>, dest: i32, w: i32) -> Option<i32> {
    // Check that there is at least one point in the path...
    if path.len() == 0 {
        return None;
    }

    let g = path.len() as i32;

    let last_point = path[path.len() - 1];
    let h = distance(last_point, dest, w);

    let f = g + h;
    Some(f)
}

fn choose_path(open_list: &mut Vec<Vec<i32>>, dest: i32, w: i32) -> Option<Vec<i32>> {
    // Setup the result data...
    let mut best_i = None;
    let mut best_f = None;

    // Iterate through the open list...
    for (i, path) in open_list.iter().enumerate() {
        // Calculate the f value for this path...
        let f = eval_path(path, dest, w);

        // Was an f value found?
        if let Some(f) = f {

            // Was there a previous best? And is this one better?
            if best_f == None || f < best_f.unwrap() {
                best_f = Some(f);
                best_i = Some(i);
            }
        }
    }

    // If no path was found, return None...
    if best_i == None || best_f == None {
        return None;
    }

    // Otherwise, pull out the best path...
    let best_path = open_list.remove(best_i.unwrap());
    Some(best_path)
}

fn main() {
    let input = parse_input(INPUT_PATH);
    
    let mut open_list = Vec::new();   
    let mut closed_list = Vec::new();   

    // Start the loop...
    loop {
        // Get the next path to check...
        let path = choose_path(&mut open_list, input.end, input.width);

        // Check if there is a path...
        if path == None {
            panic!("no path found!");
        }
        let path = path.unwrap();

        // Get the last point in the path...
        let last_point = path.get(path.len() - 1);
        if last_point == None {
            panic!("path has no last point!");
        }
        let last_point = *last_point.unwrap();

        // Check if the last point is the destination...
        if last_point == input.end {
            println!("found path: {:?}", path);
            break;
        }
        
    }
}
