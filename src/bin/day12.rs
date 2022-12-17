use std::fs;
use std::collections::{HashSet, HashMap};

#[allow(dead_code)]
const INPUT_PATH_REAL: &str = "inputs/day-12.txt";

#[allow(dead_code)]
const INPUT_PATH_EXAMPLE: &str = "inputs/day-12-example.txt";


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
    end: i32,
}

fn parse_input(path: &str) -> StartData {
    let mut grid = Vec::new();
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
            if c == 'E' {
                end = idx;
            }
            grid.push(char_to_height(c));
        }
    }

    StartData { grid, width, height, end }
}

fn check_move(from_height: i32, to_height: i32) -> bool {
    // from_height - to_height >= -1
    from_height >= to_height - 1
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

fn reconstruct_path(came_from: &HashMap<i32, i32>, start: i32, end: i32) -> Option<Vec<i32>> {
    let mut current = end;
    let mut path = Vec::new();

    while current != start {
        path.push(current);
        if let Some(next) = came_from.get(&current) {
            current = *next;
        } else {
            return None;
        }
    }

    path.push(start);
    path.reverse();
    Some(path)
}

fn pick_best_next(open_set: &mut HashSet<i32>, f_scores: &HashMap<i32, i32>) -> Option<(i32, i32)> {
    // Setup the result data...
    let mut best_p: Option<i32> = None;
    let mut best_f: Option<i32> = None;

    // Iterate through the open list...
    for point in open_set.iter() {
        // Get the f score for this point...
        let f = f_scores.get(point);

        // Was an f score found?
        if let Some(f) = f {
            let f = *f;
            let p = *point;

            // Was there a previous best? And is this one better?
            if best_f == None || f < best_f.unwrap() {
                best_f = Some(f);
                best_p = Some(p);
            }
        }
    }

    // If no path was found, return None...
    if best_p == None || best_f == None {
        return None;
    }
    let best_p = best_p.unwrap();
    let best_f = best_f.unwrap();

    // Otherwise, pull out the best path...
    open_set.remove(&best_p);
    Some((best_p, best_f))
}

fn main() {
    // Parse the input data...
    let input = parse_input(INPUT_PATH_REAL);
    
    let starting_points = input.grid
        .clone()
        .iter()
        .enumerate()
        .map(|(i, e)| (i as i32, *e))
        .filter(|(_, e)| *e == char_to_height('a'))
        .map(|(i, _)| i)
        .collect::<Vec<i32>>();
    
    let mut best_path: Option<i32> = None;
    for start in starting_points {

    
        // Initialize the data structures...
        let mut open_set: HashSet<i32> = HashSet::new();
        let mut closed_set: HashSet<i32> = HashSet::new();
        let mut g_scores: HashMap<i32, i32> = HashMap::new();
        let mut f_scores: HashMap<i32, i32> = HashMap::new();
        let mut came_from: HashMap<i32, i32> = HashMap::new();
        let mut res: Option<Vec<i32>> = None;

        // Add the start point to the open list...
        open_set.insert(start);
        g_scores.insert(start, 0);
        f_scores.insert(start, distance(start, input.end, input.width));

        // Start the loop...
        loop {
            // Get the next path to check...
            let point = pick_best_next(&mut open_set, &f_scores);
            
            // Check if there is a path...
            if point == None {
                break;
            }
            let (point, _) = point.unwrap();

            // Check if the last point is the destination...
            if point == input.end {
                res = reconstruct_path(&came_from, start, input.end);
                break;
            }

            // Add the path to the closed list...
            closed_set.insert(point);

            // Get the neighbors of the last point...
            let neighbors: Vec<i32> = get_neighbors(point, input.width, input.height)
                .into_iter()
                .filter(|n| {
                    // Get the height of the last point...
                    let from_height = input.grid[point as usize];
                    let to_height = input.grid[*n as usize];
                    check_move(from_height, to_height)
                })
                .collect();

            let this_g = g_scores.get(&point).unwrap() + 1;

            // Add the neighbors to the open list...
            for n in neighbors {
                // For this neighbor, get the previous g score and the new g score...
                let pg = g_scores.get(&n); // Previous g score
                let ng = this_g + 1; // New g scores (All distances are 1)

                // Is the new g score better?
                if pg == None || ng < *pg.unwrap() {
                    // Update the g score...
                    g_scores.insert(n, ng);

                    // Update the f score...
                    f_scores.insert(n, ng + distance(n, input.end, input.width));

                    // Update the came from...
                    came_from.insert(n, point);
                    
                    // Add the neighbor to the open list...
                    open_set.insert(n);
                }
            }
        }

        // Print the result...
        if let Some(res) = res {
            let path_len = (res.len() - 1) as i32;
            if best_path == None || path_len < best_path.unwrap() {
                best_path = Some(path_len);
            }
        } else {
            println!("No path found for start \"{}\"!", start);
        }
    }

    // Print the result...
    println!("Best path length: {}", best_path.unwrap());
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
