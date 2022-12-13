use std::collections::{VecDeque, HashSet};

const START_DELAY: i32 = 2;

#[derive(Debug, Clone, Copy)]
struct Cmd {
    delay: i32,
    amount: i32,
}

fn main() {
    let input_file = "inputs/day-10.txt";
    let raw = std::fs::read_to_string(input_file)
        .expect("couldn't read input file");

    // Read in the instructions...
    let mut instructions: VecDeque<_> = raw
        .split("\n")
        .map(|line| {
            // Is it a noop line?
            if line == "noop" {
                return None;
            }

            // Split the parts and parse the #
            let parts: Vec<_> = line.split(" ").collect();
            let cmd = parts[0];
            if cmd != "addx" {
                panic!("how did I get here?! line = \"{}\"", line);
            }

            let amount: i32 = parts[1]
                .parse()
                .expect(format!("couldn't parse number in line \"{}\"", line).as_str())
                ;

            // Return as an instruction...
            Some(amount)
        })
        .collect();

    // Setup the state...
    let save_states: HashSet<_> = vec![
         20,
         60, 
        100, 
        140, 
        180, 
        220,
    ]
        .into_iter()
        .collect();
    let mut subtotals: Vec<i32> = Vec::new();
    // let mut total = 0;
    let mut register = 1;
    let mut running_cmd: Option<Cmd> = None;

    // Start running...
    for i in 1..=220 {
        // A) Run a command (unless one is already running)...
        match running_cmd {
            Some(_) => {}, // A command is alreay running. Do nothing.
            None => {
                if let Some(ins) = instructions.pop_front() { // Is a command to run?
                    if let Some(n) = ins { // Is the command an "addx"? (vs a "noop")
                        running_cmd = Some(Cmd {
                            delay: START_DELAY,
                            amount: n,
                        });
                    }
                } else {
                    println!("[round={}] No more instructions left to run.", i)
                }
            },
        }

        // B) Save the state if specified...
        if save_states.contains(&i) {
            let signal_strength = i * register;
            subtotals.push(signal_strength);
        }

        // c) Tick any counters + update the register...
        if let Some(cmd) = running_cmd {
            if cmd.delay > 1 {
                running_cmd = Some(Cmd {
                    delay: cmd.delay - 1,
                    amount: cmd.amount,
                });
            } else {
                register += cmd.amount;
                running_cmd = None;
            }
        }
    }
    
    // Calculate the total...
    let total = subtotals
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("total = {}", total);

}
