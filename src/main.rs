#![allow(dead_code)]
#![allow(unused_imports)]

use aoc_22::{days, util};
use clap::Parser;


#[derive(Debug, Parser)]
#[command(name = "aoc-22", about = "Advent of Code 2022")]
struct Cli {
    /// Day to run
    #[arg(short, long)]
    day: u8,

    /// Part to run
    #[arg(short, long)]
    part: u8,

    /// Use the example input?
    #[arg(short, long)]
    example: bool,
}

fn main() -> Result<(), String> {
    // Parse the arguments...
    let cli = Cli::parse();

    // Pull out the day, part, and example-ness...
    let day = cli.day;
    let part = cli.part;
    let use_example = cli.example;

    // Validate the day and the part...
    if day < 1 || day > 25 {
        return Err(format!("Invalid day '{}'. Must be in range [1, 25].", day));
    }
    if part != 1 && part != 2 {
        return Err(format!("Invalid part '{}'. Must be in range [1, 2].", part));
    }

    println!("{:?}", cli);

    // Read in the input...
    let input = util::load_input(day, use_example)?;

    // Run the day...
    match day {
         1 => days::day01::run(part, &input),
         2 => days::day02::run(part, &input),
         3 => days::day03::run(part, &input),
         4 => days::day04::run(part, &input),
         5 => days::day05::run(part, &input),
         6 => days::day06::run(part, &input),
         7 => days::day07::run(part, &input),
         8 => days::day08::run(part, &input),
         9 => days::day09::run(part, &input),
        10 => days::day10::run(part, &input),
        11 => days::day11::run(part, &input),
        12 => days::day12::run(part, &input),
        13 => days::day13::run(part, &input),
        14 => days::day14::run(part, &input),
        15 => days::day15::run(part, &input),
        16 => days::day16::run(part, &input),
        17 => days::day17::run(part, &input),
        18 => days::day18::run(part, &input),
        19 => days::day19::run(part, &input),
        20 => days::day20::run(part, &input),
        21 => days::day21::run(part, &input),
        22 => days::day22::run(part, &input),
        23 => days::day23::run(part, &input),
        24 => days::day24::run(part, &input),
        25 => days::day25::run(part, &input),
        _ => unreachable!(),
    }
}
