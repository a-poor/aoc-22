use std::collections::HashSet;
use aoc_22::util::load_input;

const SAND_SOURCE_X: i32 = 500;
const SAND_SOURCE_Y: i32 = 0;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Returns a new point
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn sand_source() -> Self {
        Point {
            x: SAND_SOURCE_X,
            y: SAND_SOURCE_Y,
        }
    }

    /// Returns a Point parsed from a string in the format `<x-val>,<y-val>`
    fn from_str(txt: &str) -> Result<Self, String> {
        // Split the point & validate...
        let parts = txt.trim().split(",").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(format!("Expected `txt` to be 2 partrs joined by a \",\" but got \"{:?}\"", parts));
        }

        // Parse the x value...
        let x = match parts[0].parse::<i32>() {
            Ok(x) => x,
            Err(err) => {
                return Err(format!("failed to parse x value {} as an i32: {}", parts[0], err));
            },
        };

        // Parse the y value...
        let y = match parts[1].parse::<i32>() {
            Ok(y) => y,
            Err(err) => {
                return Err(format!("failed to parse y value {} as an i32: {}", parts[1], err));
            },
        };

        // Return the result...
        Ok(Point::new(x, y))
    }

    /// Create a sorted vector of Points going between this point 
    /// and another point's x-position. 
    fn to_x(&self, other_x: i32) -> Vec<Point> {
        // Find the starting and ending points...
        let (start, end) = if self.x <= other_x {
            (self.x, other_x)
        } else {
            (other_x, self.x)
        };

        // Iterate and return...
        (start+1..end)
            .map(|x| Point::new(x, self.y))
            .collect()
    }

    /// Create a sorted vector of Points going between this point 
    /// and another point's y-position. 
    fn to_y(&self, other_y: i32) -> Vec<Point> {
        // Find the starting and ending points...
        let (start, end) = if self.y <= other_y {
            (self.y, other_y)
        } else {
            (other_y, self.y)
        };

        // Iterate and return...
        (start+1..end)
            .map(|y| Point::new(self.x, y))
            .collect()
    }

    fn to_other(&self, other: Point) -> Result<Vec<Point>, String> {
        // Validate the different positions...
        if self.x != other.x && self.y != other.y {
            return Err(format!("other point can't be in both a different column and row. self={:?}, other={:?}", self, other));
        }

        // Otherwise, return the vecs...
        if self.x != other.x {
            Ok(self.to_x(other.x))
        } else {
            Ok(self.to_y(other.y))
        }
    }

    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y+1,
        }
    }

    fn down_left(&self) -> Point {
        Point {
            x: self.x-1,
            y: self.y+1,
        }
    }

    fn down_right(&self) -> Point {
        Point {
            x: self.x+1,
            y: self.y+1,
        }
    }
}

enum SandPos {
    Landed(Point),
    NoRoom,
    OffTheEdge,
}

struct State {
    sand_source: Point,
    resting_sand: HashSet<Point>,
    rocks: HashSet<Point>,
}

impl State {
    fn new() -> Self {
        Self { 
            sand_source: Point::sand_source(),
            resting_sand: HashSet::new(),
            rocks: HashSet::new(),
        }
    }

    fn add_rock(&mut self, point: Point) {
        self.rocks.insert(point);
    }

    fn add_rocks(&mut self, points: Vec<Point>) {
        for p in points {
            self.add_rock(p);
        }
    }

    fn grid_range(&self) -> Result<GridRange, String> {
        if self.rocks.len() == 0 {
            return Err(format!("no points in the grid"));
        }

        let mut min_x: i32 = self.sand_source.x;
        let mut max_x: i32 = self.sand_source.x;
        let mut min_y: i32 = self.sand_source.y;
        let mut max_y: i32 = self.sand_source.y;

        for point in self.rocks.iter() {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }
        
        Ok(GridRange {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }

    fn draw_grid(&self) -> Result<(), String> {
        // Get the grid's range...
        let grid_range = self.grid_range()?;

        // Draw the rocks...
        for yi in grid_range.min_y..=grid_range.max_y {
            for xi in grid_range.min_x..=grid_range.max_x {
                let this_point = Point::new(xi, yi);
                if this_point == self.sand_source {
                    print!("+");
                } else if self.rocks.contains(&this_point) {
                    print!("#");
                } else if self.resting_sand.contains(&this_point) {
                    print!("o");
                } else {
                    print!(".");
                }
            }
            println!(); // Add a newline...
        }

        // Success!
        Ok(())
    }

    fn is_point_blocked(&self, point: Point) -> bool {
        self.rocks.contains(&point) || self.resting_sand.contains(&point)
    }

    fn get_next_sand_pos(&self, current: Point) -> Option<Point> {
        // Try to move down...
        let next = current.down();
        if !self.is_point_blocked(next) {
            return Some(next);
        }

        // Otherwise, try down and to the left...
        let next = current.down_left();
        if !self.is_point_blocked(next) {
            return Some(next);
        }

        // Otherwise, try down and to the right...
        let next = current.down_right();
        if !self.is_point_blocked(next) {
            return Some(next);
        }

        // Otherwise, there's nowhere else to go...
        None
    }

    fn drop_sand_once(&mut self) -> Result<SandPos, String> {
        let grid = self.grid_range()?;

        let mut resting_sand = Some(self.sand_source);
        loop {
            let this_sand = resting_sand.unwrap();
            let next_sand = self.get_next_sand_pos(this_sand);
            
            match next_sand {
                Some(p) => {
                    // Is the sand off the edge?
                    if p.y > grid.max_y {
                        return Ok(SandPos::OffTheEdge);
                    }

                    // Set it as the new sand...
                    resting_sand = next_sand; 
                },
                None => { break; },
            }
        }

        let resting_sand = resting_sand.unwrap();
        if resting_sand == self.sand_source {
            return Ok(SandPos::NoRoom);
        }
        
        // Add the resting sand and return...
        self.resting_sand.insert(resting_sand);    
        Ok(SandPos::Landed(resting_sand))
    
    }

}

#[derive(Debug)]
struct GridRange {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn main() -> Result<(), String> {
    // Load the input data...
    let example = true;
    let raw = load_input(14, example)?;

    // Create a new grid...
    let mut state = State::new();

    // Add the rocks (by iterating through the lines)...
    for line in raw.split("\n") {
        // Split into points...
        let raw_points = line.split(" -> ");

        // Store the previous point...
        let mut last_point: Option<Point> = None;

        // Iterate through the points...
        for raw_point in raw_points {
            // Parse that point...
            let p = Point::from_str(raw_point)?;

            // Add it to the grid...
            state.add_rock(p);

            // If there's previous point... 
            if let Some(last_point) = last_point {
                // Create the vec of in-between points...
                let between = last_point.to_other(p)?;
                
                // And add them to the grid...
                state.add_rocks(between);
            }

            // Update the last point...
            last_point = Some(p);
        }
    }

    // Draw the grid...
    println!("Starting grid...");
    state.draw_grid()?;
    println!();

    // Start dropping sand...
    loop {
        match state.drop_sand_once()? {
            SandPos::Landed(_) => {},
            SandPos::NoRoom => {
                println!("No room!");
                break;
            },
            SandPos::OffTheEdge => {
                println!("Off the edge!");
                break;
            },
        }
    }

    println!("Done.");
    println!("Final grid...");
    state.draw_grid()?;
    println!();

    println!("# of resting sand units: {}", state.resting_sand.len());

    // Success!
    Ok(())
}
