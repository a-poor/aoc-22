use std::collections::VecDeque;

/// Size of the rock chamber
const CHAMBER_WIDTH: usize = 7;

/// 
const START_X_PAD: usize = 2;

/// 
const START_Y_PAD: usize = 3;

/// Number of rocks to drop
const N_ROCKS: usize = 2022;


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Move {
    Left,
    Right,
}

#[allow(dead_code)]
impl Move {
    fn parse(c: char) -> Result<Move, String> {
        match c {
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(format!("invalid char '{}'", c)),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Rock {
    HLine,
    Cross,
    LShape,
    VLine,
    Square,
}

#[allow(dead_code)]
impl Rock {
    /// Get the next rock shape in order
    fn next(&self) -> Rock {
        match self {
            Rock::HLine => Rock::Cross,
            Rock::Cross => Rock::LShape,
            Rock::LShape => Rock::VLine,
            Rock::VLine => Rock::Square,
            Rock::Square => Rock::Cross,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct State {
    /// Input data; List of L/R moves to make
    moves: VecDeque<Move>,
    
    /// Settled rocks (index 0 is the base)
    resting_rocks: Vec<Vec<bool>>,

    /// Next rock to drop in the queue
    next_rock: Rock,
}

#[allow(dead_code)]
impl State {
    fn new(raw: &str) -> Result<State, String> {
        let mut moves = VecDeque::new();
        for c in raw.chars() {
            let m = Move::parse(c)?;
            moves.push_back(m);
        }
        Ok(State {
            moves,
            resting_rocks: vec![
                (0..CHAMBER_WIDTH).map(|_| true).collect()
            ],
            next_rock: Rock::HLine,
        })
    }

    fn get_top_rock_height(&self) -> i32 {
        // TODO - The inner loop may cause an issue w/ pointers?
        for (i, row) in self.resting_rocks.iter().rev() {
            for r in row.iter() {
                if r {
                    return i
                }
            }
        }
        panic!("No valid resting rock");
    }

    fn drop_rock(&mut self) -> Result<(), String> {
        // Get the rock to be dropped...
        let rock = self.next_rock;

        // Drop the appropriate rock...
        match rock {
            Rock::HLine => self.drop_hline()?,
            Rock::Cross => self.drop_cross()?,
            Rock::LShape => self.drop_lshape()?,
            Rock::VLine => self.drop_vline()?,
            Rock::Square => self.drop_square()?,
        }
        
        // Move to the next rock...
        self.next_rock = rock.next();
    }

    fn drop_hline(&mut self) -> Result<(), String> {
        // Init the starting position...
        let width = 4;
        let mut xpos = START_X_PAD;
        let mut ypos = START_Y_PAD + self.get_top_rock_height();

        // Start looping...
        loop {
            // Get the next move...
            let rmove = self.moves.pop_front();

            // Attemtp to move L/R...
            match rmove {
                Move::Right => {
                    // - If already at the right edge, do nothing.
                    // - If path blocked, do nothing...
                    // - Otherwise, move left
                    if (
                        xpos+width < CHAMBER_WIDTH && (
                            ypos >= self.resting_rocks.len()
                            || !self.resting_rocks[ypos][xpos+1]
                        )
                    ) {
                        xpos += 1;
                    }
                },
                Move::Left => {
                    // - If already at the left edge, do nothing.
                    // - If path blocked, do nothing...
                    // - Otherwise, move left
                    if (
                        xpos > 0 && (
                            ypos >= self.resting_rocks.len()
                            || !self.resting_rocks[ypos][xpos-1]
                        ) 
                    ) {
                        xpos -= 1; // Move left!
                    }
                },
            }

            // Attempt to move down one...
            let mut hits = false;
            for i in 0..width {
                let x = xpos + i;
                if self.resting_rocks[ypos+1][x] {
                    hits = true;
                    break;
                }
            }
            if hits {
                // Add the current resting positoon as resting blocks...
                if 
            } else {
                ypos -= 1;
            }
        }

        // It worked!
        Ok(())
    }

    fn drop_cross(&mut self) {}

    fn drop_lshape(&mut self) {}

    fn drop_vline(&mut self) {}

    fn drop_square(&mut self) {}
}

fn main() -> Result<(), String> {
    // Load the input data...
    let raw = aoc_22::util::load_input(17, true)?;

    // Success!
    Ok(())
}


