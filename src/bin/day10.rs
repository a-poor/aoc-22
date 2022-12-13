use std::collections::{VecDeque, HashSet};

const START_DELAY: i32 = 2;

#[derive(Debug, Clone, Copy)]
struct Cmd {
    delay: i32,
    amount: i32,
}

fn main() {
    // let input_file = "inputs/day-10.txt";
    // let raw = std::fs::read_to_string(input_file)
    //     .expect("couldn't read input file");

    let raw = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

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
