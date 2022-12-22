
use std::fs;

pub fn load_input(day: u8, example: bool) -> Result<String, String> {
    // Format the filename...
    let filename = if example {
        format!("inputs/day-{}-example.txt", day)
    } else {
        format!("inputs/day-{}.txt", day)
    };

    // Read in the file and return...
    match fs::read_to_string(filename) {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("failed to read input file: {}", err)),
    }
}


pub const FAVE_NUM: i32 = 2;