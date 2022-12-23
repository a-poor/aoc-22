#[allow(dead_code)]
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.abs() + dy.abs()
    }

    fn points_in_range(&self, distance: i32) -> HashSet<Point> {
        let mut points = HashSet::new();
        
        // NOTE: This could be a lot cleaner, but I'm a few days behind on AoC-22...
        for x in self.x - distance..=self.x + distance {
            for y in self.y - distance..=self.y + distance {
                let p = Point::new(x, y);
                if self.dist(&p) <= distance {
                    points.insert(p);
                }
            }
        }

        points
    }
}

#[derive(Debug, Clone, Copy)]
struct DataPoint {
    sensor: Point,
    closest_beacon: Point,
}

impl DataPoint {
    fn new(sensor: Point, closest_beacon: Point) -> Self {
        Self { sensor, closest_beacon }
    }

    fn dist_to_beacon(&self) -> i32 {
        self.sensor.dist(&self.closest_beacon)
    }
}

fn parse_line(line: &str) -> Result<DataPoint, String> {
    let re = regex::Regex::new(r"Sensor at x=([-0-9]+), y=([-0-9]+): closest beacon is at x=([-0-9]+), y=([-0-9]+)")
        .expect("invalid regex");
    let caps = match re.captures(line) {
        Some(caps) => caps,
        None => {
            return Err(format!("no captures found in line \"{}\"", line));
        }
    };
 
    let sx = caps[1].parse::<i32>().unwrap();
    let sy = caps[2].parse::<i32>().unwrap();
    let bx = caps[3].parse::<i32>().unwrap();
    let by = caps[4].parse::<i32>().unwrap();

    let s = Point::new(sx, sy);
    let b = Point::new(bx, by);
    Ok(DataPoint::new(s, b))
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct GridRange {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

struct State {
    data_points: Vec<DataPoint>,
}

#[allow(dead_code)]
impl State {
    fn new() -> State {
        State { data_points: Vec::new() }
    }

    fn grid_range(&self) -> Result<GridRange, String> {
        if self.data_points.len() == 0 {
            return Err(format!("empty data points"));
        }
        
        let dp0 = self.data_points[0];
        let mut x_min = dp0.sensor.x;
        let mut x_max = dp0.sensor.x;
        let mut y_min = dp0.sensor.x;
        let mut y_max = dp0.sensor.x;
    
        for dp in self.data_points.iter() {
            // Find the distance from the sensor to the beacon...
            // NOTE: Add one to be safe...
            let dist = dp.dist_to_beacon() + 1;

            // Check left...
            let x = dp.sensor.x - dist;
            if x < x_min {
                x_min = x;
            }

            // Check right...
            let x = dp.sensor.x + dist;
            if x > x_max {
                x_max = x;
            }

            // Check up...
            let y = dp.sensor.y - dist;
            if y < y_min {
                y_min = y;
            }

            // Check down...
            let y = dp.sensor.y + dist;
            if y > y_max {
                y_max = y;
            }
        }

        // Format and return...
        Ok(GridRange {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

fn main() -> Result<(), String> {
    let raw = aoc_22::util::load_input(15, false)?;
    let target_y = 2_000_000;
    // let raw = aoc_22::util::load_input(15, true)?;
    // let target_y = 10;

    let mut state = State::new();
    for line in raw.split("\n") {
        let dp = parse_line(line)?;
        state.data_points.push(dp);
    }

    let grid_range = state.grid_range()?;
    println!("grid range: {:?}", grid_range);

    let sensors = state.data_points.iter().map(|dp| dp.sensor).collect::<HashSet<_>>();
    let beacons = state.data_points.iter().map(|dp| dp.closest_beacon).collect::<HashSet<_>>();

    // Get the unused spaces
    let mut unused: HashSet<Point> = HashSet::new();
    for y in grid_range.y_min..=grid_range.y_max {
        if y != target_y {
            continue;
        }

        for x in grid_range.x_min..=grid_range.x_max {
            // Define the point...
            let p = Point::new(x, y);

            // Check if the point is a sensor or beacon...
            if sensors.contains(&p) || beacons.contains(&p) {
                continue;
            }

            // Check if the point is in range of a sensor...
            let mut in_range = false;
            for dp in state.data_points.iter() {
                let s = dp.sensor;
                let b = dp.closest_beacon;
                
                if s.dist(&p) <= s.dist(&b) {
                    in_range = true;
                    break;
                }
            }
            if in_range {
                unused.insert(p);
            }
        }
    }

    let mut count = 0;
    for p in unused.iter() {
        if p.y == target_y {
            count += 1;
        }
    }
    println!("count: {}", count);

    Ok(())
}
