use std::fs;
use std::collections::{HashSet, HashMap};

const INPUT_PATH: &str = "inputs/day-12.txt";

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

struct StartData {
    grid: Vec<i32>,
    width: i32,
    height: i32,
    start: i32,
    end: i32,
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
    let height = lines.len() as i32;

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

    StartData { grid, width, height, start, end }
}

fn check_move(from_height: i32, to_height: i32) -> bool {
    from_height - to_height >= -1
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

fn get_neighbors(idx: i32, w: i32, h: i32) -> Vec<i32> {
    let mut neighbors = Vec::new();

    // Get the 2D coordinates of the point...
    let (i, j) = idx_1d_to_2d(idx, w);

    // Check the left neighbor...
    if j > 0 {
        neighbors.push(idx_2d_to_1d(i, j - 1, w));
    }

    // Check the right neighbor...
    if j < w - 1 {
        neighbors.push(idx_2d_to_1d(i, j + 1, w));
    }

    // Check the top neighbor...
    if i > 0 {
        neighbors.push(idx_2d_to_1d(i - 1, j, w));
    }

    // Check the bottom neighbor...
    if i < h - 1 {
        neighbors.push(idx_2d_to_1d(i + 1, j, w));
    }

    neighbors
}

fn main() {
    // Parse the input data...
    let input = parse_input(INPUT_PATH);
    
    // Initialize the data structures...
    let mut open_list = Vec::new();
    let mut closed_list: HashSet<i32> = HashSet::new();
    // let mut chain: HashMap<i32, i32> = HashMap::new();
    let mut res: Option<Vec<i32>> = None;

    // Add the start point to the open list...
    open_list.push(vec![input.start]);

    // Start the loop...
    loop {
        // Get the next path to check...
        let path = choose_path(&mut open_list, input.end, input.width);
        
        // Check if there is a path...
        if path == None {
            break;
        }
        let path = path.unwrap();
        // println!("Checking path: {:?}", path.clone().into_iter().map(|p| idx_1d_to_2d(p, input.width)).collect::<Vec<(i32, i32)>>());
        // println!("Open list: {:?}", open_list.clone().into_iter().map(|p| p.clone().into_iter().map(|p| idx_1d_to_2d(p, input.width)).collect::<Vec<(i32, i32)>>()).collect::<Vec<Vec<(i32, i32)>>>());
        // println!("Closed list: {:?}", closed_list.clone().into_iter().map(|p| idx_1d_to_2d(p, input.width)).collect::<Vec<(i32, i32)>>());

        // Get the last point in the path...
        let last_point = path.get(path.len() - 1);
        if last_point == None {
            break;
        }
        let last_point = *last_point.unwrap();

        // Check if the last point is the destination...
        if last_point == input.end {
            res = Some(path);
            break;
        }

        // Add the path to the closed list...
        closed_list.insert(last_point);

        // Get the neighbors of the last point...
        let neighbors = get_neighbors(last_point, input.width, input.height)
            .into_iter()
            .filter(|n| {
                // Is it already in the closed list?
                if closed_list.contains(n) { return false }

                // Get the height of the last point...
                let from_height = input.grid[last_point as usize];
                let to_height = input.grid[*n as usize];
                if !check_move(from_height, to_height) { return false }
                
                // Otherwise, it's a valid neighbor...
                true
            })
            .collect::<Vec<_>>();

        // Add the neighbors to the open list...
        for n in neighbors {
            // println!("Checking neighbor: {:?}", idx_1d_to_2d(n, input.width));
            let mut new_path = path.clone();
            new_path.push(n);
            open_list.push(new_path);
        }

        // println!();
    }

    // Print the result...
    if let Some(res) = res {
        println!("Path found!");
        println!("Path length: {}", res.len());
        println!("Path: {:?}", res);
    } else {
        println!("No path found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx_1d_to_2d() {
        assert_eq!(
            idx_1d_to_2d(0, 10),
            (0, 0),
        );
        assert_eq!(
            idx_1d_to_2d(9, 10),
            (0, 9),
        );
        assert_eq!(
            idx_1d_to_2d(9, 5),
            (1, 4),
        );
    }

    #[test]
    fn test_get_neighbors() {
        assert_eq!(
            get_neighbors(0, 10, 10),
            vec![1, 10],
        );
        assert_eq!(
            get_neighbors(6, 5, 6),
            vec![5, 7, 1, 11]
        );
    }

    #[test]
    fn test_check_move() {
        assert!(check_move(0, 0));
        assert!(check_move(1, 0));
        assert!(check_move(0, 1));
        assert!(!check_move(0, 2));
        assert!(check_move(2, 0));
    }
}
