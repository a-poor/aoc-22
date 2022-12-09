use std::fs;


const INPUT_FILE: &str = "inputs/day-01.txt";


fn read_input_from_file(path: &str) -> Result<Vec<Option<i32>>, String> {
    // Read in the file...
    let str_or_err = fs::read_to_string(path);
    
    // Check for an error...
    if let Err(err) = str_or_err {
        return Err(format!("failed to read in file at \"{}\": {}", path, err));
    }

    // Otherwise, must be a valid string (this should never panic)...
    let raw_str = str_or_err.unwrap();

    // Iterate through the lines...
    let mut res: Vec<_> = raw_str
        .trim()
        .split("\n")
        .map(|s| {
            // Trim any unwanted space on that line...
            let s = s.trim();

            // If the line is empty, return None
            if s == "" {
                return None
            }

            // Otherwise, attempty to parse it as an int...
            let n = s
                .to_string()
                .parse::<i32>()
                .unwrap();

            // Return it!
            Some(n)

        })
        .collect()
        ;

    // Push another "None" at the end of the line...
    res.push(None);

    // Return success!
    Ok(res)
}


fn do_work(data: Vec<Option<i32>>) -> Option<i32> {
    // Create a variable to store the result...
    let mut max = None;
    let mut cur = 0;

    // Iterate through the data...
    for n in data.iter() {
        match n {
            // If n is none (aka line is empty) running tally is complete.
            // Check against the max.
            None => {
                // Is the running amount bigger that the previous max?
                // Or is the previous max none?
                match max {
                    Some(maxn) => { // There is a previous max...
                        if cur > maxn { // ...and current is larger
                            max = Some(cur);
                        }
                    },
                    None => {
                        max = Some(cur);
                    },
                }
                
                // Reset the running tally
                cur = 0;
            },

            // Otherwise, add it to the tally...
            Some(n) => {
                cur += n;
            },
        }
    }
    
    // Return the result...
    max
}

fn main() {
    let input_data = read_input_from_file(INPUT_FILE);
    let input_data = input_data.expect("Failed to read input data file");

    let res = do_work(input_data);
    println!("Result: {:?}", res);
}
