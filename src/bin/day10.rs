
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
    let instructions: Vec<_> = raw
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

    // Define thr rounds to store...
    let key_rounds: Vec<usize> = vec![
         20,
         60, 
        100, 
        140, 
        180, 
        220,
    ];

    // Calculate the values at each key round...
    let sub_totals: Vec<_> = key_rounds
        .into_iter()
        .map(|i| {
            let register = instructions
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(j, _)| {
                    let j = *j as i32;
                    let i = i as i32;
                    j - 2 < i - 1
                }) // Only include values before tick i (j is 0-index, i is 1-index)
                .filter(|(_, n)| *n != None) // Only include non-None values
                .map(|(_, n)| n.expect("I though I got rid of all the 'None's"))
                .reduce(|a, b| a + b)
                .unwrap_or(0);

            let strength = i * register as usize;
            println!("{}th cycle: register={}; strength={}", i, register, strength);
            
            // Return the signal strength...
            (i, i * register as usize)
        })
        .collect();

    // Sum the total...
    let total = sub_totals
        .into_iter()
        .map(|(_, n)| n)
        .reduce(|a, b| a + b)
        .unwrap_or(1);
    println!("total = {}", total);

}
