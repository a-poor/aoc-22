use std::collections::{HashSet, HashMap};

const ADDX_WAIT: i32 = 2;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i32),
}

fn main() {
    let input_file = "inputs/day-10.txt";
    let raw = std::fs::read_to_string(input_file)
        .expect("couldn't read input file");

    // Read in the instructions...
    let instructions: Vec<Instruction> = raw
        .split("\n")
        .map(|line| {
            // Is it a noop line?
            if line == "noop" {
                return Instruction::NoOp;
            }

            // Split the parts and parse the #
            let parts: Vec<_> = line.split(" ").collect();
            let amount: i32 = parts[1]
                .parse()
                .expect(format!("couldn't parse number in line \"{}\"", line).as_str())
                ;

            // Return as an instruction...
            Instruction::AddX(amount)
        })
        .collect();

    // Setup the running state...
    let mut register: i32 = 1;
    let mut running_cmds: Box<HashMap<i32, i32>> = Box::new(HashMap::new());

    //
    let key_rounds: HashSet<usize> = vec![
         20,
         60, 
        100, 
        140, 
        180, 
        220,
    ]
        .into_iter()
        .collect();
    let mut total = 0;
    
    // Iterate through the instructions...
    for i in 0..250 {
        if let Some(ins) = instructions.get(i) {
            // A) Start the cycle...
            let mut n = match ins {
                Instruction::NoOp => 0,
                Instruction::AddX(n) => *n,
            };
            if let Some(prev) = running_cmds.get(&ADDX_WAIT) {
                n += prev;
                unreachable!();
            }
            running_cmds.insert(ADDX_WAIT, n);
        }


        // B) During the cycle...
        let cycle = i + 1;
        if key_rounds.contains(&cycle) {
            total += register * cycle as i32;
        }

        // C) End of the cycle...
        // > Decrement each of the counts... 
        let mut next_state = HashMap::new();
        for (k, v) in running_cmds.into_iter() {
            next_state.insert(k-1, v);
        }
        running_cmds = Box::new(next_state);

        // > If any are <= 0, add them to the register...
        if let Some(n) = running_cmds.get(&0) {
            register += *n;
        }
    }

    println!("total = {}", total);

}
