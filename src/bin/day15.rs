#[allow(dead_code)]
use std::collections::HashSet;

const TUNING_FREQ_MULT: i128 = 4_000_000;

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

    fn tuning_freq(&self) -> i128 {
        let x = self.x as i128;
        let y = self.y as i128;
        (x * TUNING_FREQ_MULT) + y
    }

    fn iter_points(&self, other: &Point) -> PointRange {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);
        PointRange {
            from: Point::new(min_x, min_y),
            to: Point::new(max_x, max_y),
            current: Point::new(min_x, min_y),
        }
    }
}

struct PointRange {
    from: Point,
    to: Point,
    current: Point,
}

impl PointRange {
    fn size(&self) -> i128 {
        let dx = self.to.x - self.from.x;
        let dy = self.to.y - self.from.y;
        (dx as i128) * (dy as i128)
    }
}

impl Iterator for PointRange {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x > self.to.x {
            self.current.x = self.from.x;
            self.current.y += 1;
        }

        if self.current.y > self.to.y {
            return None;
        }

        let ret = self.current;
        self.current.x += 1;
        Some(ret)
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

struct IterRadius {
    center: Point,
    radius: i32,
    dx: i32,
    dy: i32,
    current: Point,
}

impl IterRadius {
    fn new(center: Point, radius: i32) -> Self {
        let current = Point::new(center.x - radius, center.y);
        Self {
            center,
            radius,
            current,
            dx: -1,
            dy: 1,
        }
    }
}

impl Iterator for IterRadius {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current.x - self.center.x).abs() > self.radius {
            self.dx *= -1;
        }
        if (self.current.y - self.center.y).abs() > self.radius {
            self.dy *= -1;
        }

        self.current.x += self.dx;
        self.current.y += self.dy;

        let ret = self.current;
        if ret == Point::new(self.center.x - self.radius, self.center.y) {
            return None;
        }

        Some(ret)
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
    let min = 0;
    let max = 4_000_000;
    // let raw = aoc_22::util::load_input(15, true)?;
    // let min =  0;
    // let max = 20;
    
    let search_min = Point::new(min, min);
    let search_max = Point::new(max, max);

    let mut state = State::new();
    for line in raw.split("\n") {
        let dp = parse_line(line)?;
        state.data_points.push(dp);
    }

    let grid_range = state.grid_range()?;
    println!("grid range: {:?}", grid_range);

    let sensors = state.data_points.iter().map(|dp| dp.sensor).collect::<HashSet<_>>();
    let beacons = state.data_points.iter().map(|dp| dp.closest_beacon).collect::<HashSet<_>>();

    for dp in state.data_points.clone() {
        // Get the distance to the closest beacon...
        let dist_to_closest_beacon = dp.dist_to_beacon();

        // Iterate through the points just outside the radius of the closest beacon...
        for p in IterRadius::new(dp.sensor, dist_to_closest_beacon + 1) {
            if p.x < search_min.x || p.x > search_max.x {
                continue;
            }
            if p.y < search_min.y || p.y > search_max.y {
                continue;
            }
            
            if sensors.contains(&p) || beacons.contains(&p) {
                continue;
            }
            
            // Iterate through the OTHER sensors and check if it's within their radius...
            let mut within_radius = false;
            for other_dp in state.data_points.clone() {
                if other_dp.sensor == dp.sensor {
                    continue;
                }

                // Check if the point is within the radius of the other sensor...
                let dist_to_other_sensor = other_dp.sensor.dist(&p);
                if dist_to_other_sensor <= other_dp.dist_to_beacon() {
                    // println!("point {:?} is within the radius of {:?} (dist: {})", p, other_dp.sensor, dist_to_other_sensor);
                    within_radius = true;
                    break;
                }
            }
            if !within_radius {
                println!("point {:?} is not within the radius of any other sensor. tuning_freq = {}", p, p.tuning_freq());
                return Ok(());
            }
        }
    }

    Ok(())
}
