/// -- Notes ------------
/// * Release as much pressure as possible in the allotted time 
/// * 30m total
/// * 1m to open a valve  
/// * 1m to go through a tunnel
/// * Can only open a valve once...

#[allow(dead_code)]
use std::collections::{HashMap, HashSet};
use rand::Rng;
use rand::rngs::ThreadRng;

const START_TIME: i32 = 30;


#[allow(dead_code)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

#[allow(dead_code)]
impl Valve {
    fn new(name: String, flow_rate: i32, tunnels: Vec<String>) -> Valve {
        Valve { name, flow_rate, tunnels }
    }

    fn parse_line(line: &str) -> Valve {
        let re = regex::Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z]+(, [A-Z]+)*)").unwrap();
        let caps = re
            .captures(line)
            .expect("no captures found");
        let name = caps
            .get(1)
            .expect("couldnt' find valve name match")
            .as_str()
            .to_string();
        let flow_rate = caps
            .get(2)
            .expect("couldn't get flow rate match")
            .as_str()
            .parse::<i32>()
            .expect("couldn't parse flow rate as int");
        let tunnels = caps
            .get(3)
            .expect("couldn't get tunnels match")
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        Valve::new(name, flow_rate, tunnels)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Move {
    OpenValve,
    GoThroughTunnel(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct GameState {
    time: i32,
    pressure: i32,
    opened: HashSet<String>,
}

#[allow(dead_code)]
impl GameState {
    fn new() -> GameState {
        GameState { pressure: 0, time: START_TIME, opened: HashSet::new() }
    }

    fn is_over(&self) -> bool {
        self.time <= 0
    }

    fn is_open(&self, valve: &Valve) -> bool {
        self.opened.contains(&valve.name)
    }

    fn tick(&mut self) {
        self.time -= 1;
    }

    fn open_valve(&mut self, valve: &Valve) -> i32 {
        if self.is_open(valve) {
            return 0;
        }
        
        self.opened.insert(valve.name.clone());
        self.pressure += valve.flow_rate;
        valve.flow_rate
    }
}

struct GameSetup {
    /// How to update the existing 
    learning_rate: f32,

    /// How likely we are to explore new options 
    /// vs exploit what we already know
    epsilon: f32,
}

struct GameResult {
    pressure_released: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct StateMamanger {
    valves: Vec<Valve>,
    memory: HashMap<String, HashMap<Move, f32>>,
}

#[allow(dead_code)]
impl StateMamanger {
    fn new(valves: Vec<Valve>) -> StateMamanger {
        // Initialize the empty memory...
        let mut memory = HashMap::new();

        // For each valve...
        for valve in &valves {
            // Create a map of moves to their Q values...
            let mut moves = HashMap::new();
            
            // Add the option to open the valve...
            moves.insert(Move::OpenValve, 0.0);

            // Add the option to go through each tunnel...
            for tunnel in &valve.tunnels {
                moves.insert(Move::GoThroughTunnel(tunnel.clone()), 0.0);
            }

            // Add the map to the memory...
            memory.insert(valve.name.clone(), moves);
        }

        // Initialize and return...
        StateMamanger { 
            valves,
            memory,
        }
    }

    fn run_a_game(&mut self, setup: GameSetup) -> GameResult {
        // Create the random number generator...
        let mut rng = rand::thread_rng();

        // Create the game state...
        let mut state = GameState::new();

        // While the game is not over...
        while !state.is_over() {

            // For each valve...
            for valve in &self.valves {

                // If the valve is not open...
                if state.is_open(valve) {

                    // Get the Q values for the valve...
                    let q_values = self.memory.get(&valve.name).unwrap();

                    // Get the best move...
                    let best_move = self.get_best_move(q_values, &mut rng, setup.epsilon);

                    // Perform the move...
                    match best_move {
                        Move::OpenValve => {
                            // Open the valve...
                            self.opened.insert(valve.name.clone());

                            // Update the game state...
                            state.dec_time();
                            state.inc_pressure(valve.flow_rate);
                        },
                        Move::GoThroughTunnel(tunnel) => {
                            // Update the game state...
                            state.dec_time();
                        },
                    }
                }
            }
        }

        // Return the result...
        GameResult { pressure_released: state.pressure }
    }

    fn get_best_move(&self) -> Move {
        Move::OpenValve
    }
}

fn main() -> Result<(), String> {
    let raw = aoc_22::util::load_input(16, true)?;
    let valves = raw.split("\n").map(|line| Valve::parse_line(line)).collect();
    let state = StateMamanger::new(valves);

    for valve in &state.valves {
        println!("{:?}", valve);
    }

    Ok(())
}
