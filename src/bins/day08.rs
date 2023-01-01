
const INPUT_FILE: &str = "inputs/day-08.txt";

fn main() {
    let raw = std::fs::read_to_string(INPUT_FILE).expect("couldnt' read input file");
    
    // Split the input up into a grid of ints
    let grid: Vec<_> = raw
        .split("\n")
        .map(|line| {
            line
                .chars()
                .map(|c| c
                    .to_string()
                    .parse::<u8>()
                    .expect(
                        format!("can't turn that char into a u8 {}", c)
                        .as_str()
                    )
                )
                .collect::<Vec<_>>()
        })
        .collect();

    // Get the grid widths and heights...
    let grid_height = grid.len();
    let grid_width = grid.get(0).expect("grid has 0 rows!").len();

    // Keep track of the best score...
    let mut best_score = 0;

    // Iterate through the grid of trees...
    for i in 0..grid_height {
        for j in 0..grid_width {
            // Get this tree's (tree a) height...
            let ha = grid[i][j];

            // Look left...
            let mut left_dist = 0;
            for k in (0..j).rev() {
                // Get the height of tree b...
                let hb = grid[i][k];
                
                // Is tree b shorter than tree a?
                left_dist += 1;
                if hb >= ha {
                    break;
                }
            }

            // Look right...
            let mut right_dist = 0;
            for k in j+1..grid_width {
                // Get the height of tree b...
                let hb = grid[i][k];
                
                // Is tree b shorter than tree a?
                right_dist += 1;
                if hb >= ha {
                    break;
                }
            }

            // Look up...
            let mut up_dist = 0;
            for k in (0..i).rev() {
                // Get the height of tree b...
                let hb = grid[k][j];
                
                // Is tree b shorter than tree a?
                up_dist += 1;
                if hb >= ha {
                    break;
                }
            }

            // Look down...
            let mut down_dist = 0;
            for k in i+1..grid_height {
                // Get the height of tree b...
                let hb = grid[k][j];
                
                // Is tree b shorter than tree a?
                down_dist += 1;
                if hb >= ha {
                    break;
                }
            }

            // Calculate the total score...
            let score = left_dist * right_dist * up_dist * down_dist;

            // Is that better?
            if score > best_score {
                best_score = score;
            }
        }
    }

    // Print the best score...
    println!("best score = {}", best_score);

}
