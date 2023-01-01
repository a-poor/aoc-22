#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{VecDeque, HashSet};

/// Width of the rock chamber
const CHAMBER_WIDTH: usize = 7;

/// Left padding for the starting position
/// of each new rock dropped.
const START_X_PAD: usize = 2;

/// Starting distance from the top of the
/// resting rocks to the bottom of each
/// new rock dropped.
const START_Y_PAD: usize = 3;

/// Number of rocks to drop
const N_ROCKS: usize = 2022;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn parse(c: char) -> Result<Move, String> {
        match c {
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(format!("invalid char '{}'", c)),
        }
    }
}

/// A 2D point with integer x and y coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point { x: i32, y: i32 }

impl Point {
    /// Create a new point from the given x and y coordinates.
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_move(m: Move) -> Self {
        match m {
            Move::Left => Self::new(-1, 0),
            Move::Right => Self::new(1, 0),
        }
    }

    /// Move the point by the given point's x and y amounts.
    fn add(&self, other: Point) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Move the point by the given x and y amounts.
    fn addn(&self, x: i32, y: i32) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    HLine,
    Cross,
    LShape,
    VLine,
    Square,
}

impl RockType {
    fn next(&self) -> Self {
        match self {
            RockType::HLine => RockType::Cross,
            RockType::Cross => RockType::LShape,
            RockType::LShape => RockType::VLine,
            RockType::VLine => RockType::Square,
            RockType::Square => RockType::HLine,
        }
    }
}

/// A rock is one of the droppable rocks.
#[derive(Debug, Clone, PartialEq)]
struct Rock {
    points: HashSet<Point>,
}

impl Rock {
    /// Create a new rock from a set of points.
    fn new(points: HashSet<Point>) -> Self {
        Self { points }
    }

    /// Create a new rock of the given type.
    fn new_by_type(rtype: RockType, point: Point) -> Self {
        match rtype {
            RockType::HLine => Self::new_hline(point),
            RockType::Cross => Self::new_cross(point),
            RockType::LShape => Self::new_lshape(point),
            RockType::VLine => Self::new_vline(point),
            RockType::Square => Self::new_square(point),
        }
    }

    /// Create a new horizontal line rock
    /// from the given anchor point.
    fn new_hline(point: Point) -> Self {
        // Create a set of points...
        let mut points = HashSet::new();
        
        // Add the points for the following shape...
        // >  ####
        points.insert(point.addn(0, 0));
        points.insert(point.addn(1, 0));
        points.insert(point.addn(2, 0));
        points.insert(point.addn(3, 0));

        // Return the new rock...
        Self::new(points)
    }

    /// Create a new "+"-shaped rock from the
    /// given anchor point.
    fn new_cross(point: Point) -> Self {
        // Create a set of points...
        let mut points = HashSet::new();
        
        // Add the points for the following shape...
        // >   #
        // >  ###
        // >  .#
        points.insert(point.addn(1, 0));
        points.insert(point.addn(0, 1));
        points.insert(point.addn(1, 1));
        points.insert(point.addn(2, 1));
        points.insert(point.addn(1, 2));

        // Return the new rock...
        Self::new(points)
    }

    /// Create a new backwards "L"-shaped rock 
    /// from the given anchor point.
    fn new_lshape(point: Point) -> Self {
        // Create a set of points...
        let mut points = HashSet::new();
        
        // Add the points for the following shape...
        // >  ..#
        // >  ..#
        // >  ###
        points.insert(point.addn(0, 0));
        points.insert(point.addn(1, 0));
        points.insert(point.addn(2, 0));
        points.insert(point.addn(2, 1));
        points.insert(point.addn(2, 2));

        // Return the new rock...
        Self::new(points)
    }

    /// Create a new vertical line rock from
    /// the given anchor point.
    fn new_vline(point: Point) -> Self {
        // Create a set of points...
        let mut points = HashSet::new();
        
        // Add the points for the following shape...
        // >  #
        // >  #
        // >  #
        // >  #
        points.insert(point.addn(0, 0));
        points.insert(point.addn(0, 1));
        points.insert(point.addn(0, 2));
        points.insert(point.addn(0, 3));

        // Return the new rock...
        Self::new(points)
    }

    /// Create a new square rock from the
    /// given anchor point.
    fn new_square(point: Point) -> Self {
        // Create a set of points...
        let mut points = HashSet::new();
        
        // Add the points for the following shape...
        // >  ##
        // >  ##
        points.insert(point.addn(0, 0));
        points.insert(point.addn(1, 0));
        points.insert(point.addn(0, 1));
        points.insert(point.addn(1, 1));

        // Return the new rock...
        Self::new(points)
    }

    fn move_points_mut(&mut self, p: Point) {
        self.points = self.points
            .iter()
            .map(|point| point.add(p))
            .collect();
    }

    fn move_points(&self, p: Point) -> Self {
        // Create a new set of points...
        let mut points = HashSet::new();

        // Move each point...
        for point in self.points.iter() {
            points.insert(point.add(p));
        }

        // Return the new rock...
        Self::new(points)
    }
}

#[derive(Debug, Clone)]
struct State {
    moves: Vec<Move>,
    move_pos: usize,
    next_rock: RockType,
    resting_points: HashSet<Point>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
}

impl State {
    /// Create a new game state manager.
    fn new(moves: Vec<Move>) -> Self {
        Self { 
            moves,
            move_pos: 0,
            next_rock: RockType::HLine,
            resting_points: HashSet::new(),
            min_x: 0,
            max_x: CHAMBER_WIDTH - 1,
            min_y: 0,
        }
    }

    /// Add the points from the given rock to the
    /// set of resting points.
    fn add_points_from_rocks(&mut self, rock: &Rock) {
        for point in rock.points.iter() {
            self.resting_points.insert(point.clone());
        }
    }

    /// Check if the given rock intersects with any
    /// of the existing, resting points.
    fn intersects(&self, rock: &Rock) -> bool {
        // For each point in the rock...
        for point in rock.points.iter() {
            // If the point intersects with a resting point...
            if self.resting_points.contains(point) {
                // Stop here.
                return true;
            }
        }

        // Doesn't intersect!
        false
    }

    /// Get the next rock type and increment the
    /// internal rock-type counter.
    fn get_next_rock_type(&mut self) -> RockType {
        let rock = self.next_rock;
        self.next_rock = rock.next();
        rock
    }

    /// Get the next move from the queue.
    fn get_next_move(&mut self) -> Move {
        // Get the next move...
        let m = self.moves[self.move_pos];

        // Increment the move position (and wrap)...
        self.move_pos += 1;
        if self.move_pos >= self.moves.len() {
            self.move_pos = 0;
        }

        // Return the move...
        m
    }

    fn get_max_y(&self) -> i32 {
        self.resting_points.iter()
            .map(|p| p.y)
            .max()
            .unwrap_or(-1)
    }

    fn get_next_rock_pos(&self) -> Point {
        // Get the max y value up to this point...
        let max_y = self.get_max_y();

        // Return the next rock position, with padding...
        Point::new(
            START_X_PAD as i32,
            max_y + 1 + START_Y_PAD as i32,
        )
    }

    /// Get the next rock to drop.
    /// 
    /// Increments the internal rock-type counter.
    fn get_next_rock(&mut self) -> Rock {
        let pos = self.get_next_rock_pos();
        let rock_type = self.get_next_rock_type();
        Rock::new_by_type(rock_type, pos)
    }

    fn draw_state(&self, rock: Option<Rock>) {
        let max_y = self.get_max_y() + 10;
        for i in (0..max_y).rev() {
            print!("|");
            for j in 0..CHAMBER_WIDTH {
                let p = Point::new(j as i32, i as i32);

                if rock.is_some() && rock.as_ref().unwrap().points.contains(&p) {
                    print!("@");
                } else if self.resting_points.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("| {}", i);
        }
        
        println!("+{}+", "-".repeat(CHAMBER_WIDTH));
    }

    fn try_move_x(&self, rock: &Rock, m: Move) -> Option<Rock> {
        // Get the new position...
        let move_point = Point::from_move(m);

        // Apply the move to the rock...
        let new_rock = rock.move_points(move_point);

        // Check each of the rock's new points...
        for point in new_rock.points.iter() {
            // Is it out of bounds on the left or right?
            if point.x < self.min_x as i32 || point.x > self.max_x as i32 {
                return None;
            }

            // Does it intersect with any settled points?
            if self.resting_points.contains(point) {
                return None;
            }
        }

        // Otherwise, success! Return the new rock...
        Some(new_rock)
    }

    fn try_move_down(&self, rock: &Rock) -> Option<Rock> {
        // Get the new rock...
        let new_rock = rock.move_points(Point::new(0, -1));

        // Check each of the rock's points...
        for point in new_rock.points.iter() {
            // Is it out of bounds on the bottom?
            if point.y < self.min_y as i32 {
                return None;
            }

            // Does it intersect with any settled points?
            if self.resting_points.contains(point) {
                return None;
            }
        }

        // Otherwise, success! Return the new rock...
        Some(new_rock)
    }

    fn drop_next_rock(&mut self) {
        // Get the next rock to be dropped...
        let mut rock = self.get_next_rock();

        // self.draw_state(Some(rock.clone()));

        // Iterate until the rock comes to rest...
        loop {
            // Get the next move, if any...
            let m = self.get_next_move();
            // println!("MOVE={:?}", m);

            // Try to move the rock left/right...
            // If it can't be moved l/r, that's fine.
            let next_rock = self.try_move_x(&rock, m);
            if let Some(next_rock) = next_rock {
                rock = next_rock.clone();
            }

            // Try to move the rock down...
            let next_rock = self.try_move_down(&rock);
            if let Some(next_rock) = next_rock {
                rock = next_rock.clone();
            } else {
                // The rock has come to rest!
                self.add_points_from_rocks(&rock);
                break;
            }

            // self.draw_state(Some(rock.clone()));
        }

        // self.draw_state(None);
    }
}

fn main() -> Result<(), String> {
    // Load the input data...
    let raw = aoc_22::util::load_input(17, false)?;

    // Parse the input data...
    let moves: Vec<Move> = raw
        .chars()
        .map(|c| Move::parse(c).expect(format!("'{}' isn't a valid char", c).as_str()))
        .collect();

    // Create the game state manager...
    let mut state = State::new(moves);

    // Drop the n rocks...
    for _ in 0..N_ROCKS {
        state.drop_next_rock();
    }

    // How tall are the resting rocks?
    let max_y = state.get_max_y() + 1;
    println!("max_y = {}", max_y);

    // Success!
    Ok(())
}


