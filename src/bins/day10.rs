use std::collections::{VecDeque};

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
    let mut register = 2;
    let mut running_cmd: Option<Cmd> = None;

    let mut res = String::new();

    // Start running...
    for i in 1..=240 {
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
                }
            },
        }

        
        // // B) Save the state if specified...
        let j = (i-1) % 40 + 1;
        if register-1 == j || register == j || register+1 == j {
            res = format!("{}#", res);
        } else {
            res = format!("{}.", res);
        }
        if i % 40 == 0 {
            res = format!("{}\n", res);
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

    println!("{}", res);

}
