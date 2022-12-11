/// --- Part 1 Notes ---
/// - There is a grid of trees
/// - There is one tree per character in the grid. 
/// - Each character (digit) represents the height of the tree in that space.
/// - Only trees that can't be seen from outside the grid (top, bottom, left, or right) are marked as good
/// - A tree is considered visible from a side if the trees between it and the outside are shorter than it
/// - Q: Count the number of trees not visible from the outside!

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

    // Create a result grid. All trees default to `visible=false`
    let mut res: Vec<Vec<bool>> = (0..grid_height)
        .map(|_| (0..grid_width)
            .map(|_| false)
            .collect()
        )
        .collect()
        ;

    // Run through in each direction (l->r, r->l, t->b, b->t), "crossing out" trees that are visible...

    // Left-to-right...
    for i in 0..grid_height {
        let mut running_height: Option<u8> = None;
        
        for j in 0..grid_width {
            // Get the tree height at that index level...
            let tree_height = grid[i][j];

            // Was there a previously taller tree (or was that the first tree)?
            if let Some(rh) = running_height {
                // Is that tree taller than the previously tallest tree?
                if tree_height > rh {
                    running_height = Some(tree_height);
                    res[i][j] = true;
                }
            } else {
                // This was the first tree, automatically visible and the new tallest...
                running_height = Some(tree_height);
                res[i][j] = true;
            }
        }
    }

    // Right-to-left...
    for i in 0..grid_height {
        // Reset the running tallest tree...
        let mut running_height: Option<u8> = None;

        for j in (0..grid_width).rev() {
            // Get the tree height at that index level...
            let tree_height = grid[i][j];
            
            // Was there a previously taller tree (or was that the first tree)?
            if let Some(rh) = running_height {
                // Is that tree taller than the previously tallest tree?
                if tree_height > rh {
                    running_height = Some(tree_height);
                    res[i][j] = true;

                }

            } else {
                // This was the first tree, automatically visible and the new tallest...
                running_height = Some(tree_height);
                res[i][j] = true;

            }
        }
    }

    // Top-to-bottom...
    for j in 0..grid_width {
        let mut running_height: Option<u8> = None;

        for i in 0..grid_height {
            // Get the tree height at that index level...
            let tree_height = grid[i][j];

            // Was there a previously taller tree (or was that the first tree)?
            if let Some(rh) = running_height {
                // Is that tree taller than the previously tallest tree?
                if tree_height > rh {
                    running_height = Some(tree_height);
                    res[i][j] = true;
                }

            } else {
                // This was the first tree, automatically visible and the new tallest...
                running_height = Some(tree_height);
                res[i][j] = true;

            }
        }
    }

    // Bottom-to-top...
    for j in 0..grid_width {
        let mut running_height: Option<u8> = None;
        
        for i in (0..grid_height).rev() {
            let tree_height = grid[i][j];

            // Was there a previously taller tree (or was that the first tree)?
            if let Some(rh) = running_height {
                // Is that tree taller than the previously tallest tree?
                if tree_height > rh {
                    running_height = Some(tree_height);
                    res[i][j] = true;
                }
            } else {
                // This was the first tree, automatically visible and the new tallest...
                running_height = Some(tree_height);
                res[i][j] = true;
            }
        }
    }


    // Count the visible trees
    let count = res
        .into_iter()
        .map(|row| row
            .into_iter()
            .map(|v| if v { 1 } else { 0 })
            .reduce(|a, b| a + b)
            .expect("empty row!")
        )
        .reduce(|a, b| a + b)
        .expect("empty columns!");

    println!("found {} visible trees", count);
    println!("found {} invisible trees", (grid_width * grid_height) - count);

}
