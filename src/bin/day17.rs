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
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn move(&self, dx: i32, dy: i32) -> Point {
        Point::new(self.x + dx, self.y + dy)
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

}

fn main() -> Result<(), String> {
    // Load the input data...
    let raw = aoc_22::util::load_input(17, true)?;

    // Success!
    Ok(())
}


