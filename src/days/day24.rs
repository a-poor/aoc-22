pub mod part1;
pub mod part2;

pub fn run(part: u8, input: &str) -> Result<(), String> {
    match part {
        1 => part1::run(input),
        2 => part2::run(input),
        _ => Err(format!("Invalid part '{}'. Must be in range [1, 2].", part)),
    }
}

