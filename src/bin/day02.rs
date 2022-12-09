use std::{cmp::Ordering, fs};

const INPUT_FILE: &str = "inputs/day-02.txt";

#[derive(Debug, PartialEq, Eq, Ord)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Rock) => Some(Ordering::Equal),
            (Move::Rock, Move::Paper) => Some(Ordering::Less),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),

            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Paper, Move::Paper) => Some(Ordering::Equal),
            (Move::Paper, Move::Scissors) => Some(Ordering::Less),

            (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Scissors, Move::Paper) => Some(Ordering::Greater),
            (Move::Scissors, Move::Scissors) => Some(Ordering::Equal),
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Tie,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Loss => 0,
        }
    }
}

struct Round {
    my_move: Move,
    their_move: Move,
}

impl Round {
    fn new(them: &str, me: &str) -> Self {
        Round {
            my_move: parse_my_move(me),
            their_move: parse_their_move(them),
        }
    }

    fn outcome(&self) -> Outcome {
        if self.my_move == self.their_move {
            Outcome::Tie
        } else if self.my_move > self.their_move {
            Outcome::Win
        }  else {
            Outcome::Loss
        }
    }

    fn score(&self) -> i32 {
        self.outcome().score() + self.my_move.score()
    }
}

fn parse_their_move(c: &str) -> Move {
    match c {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("unknown their move code \"{}\"", c),
    }
}

fn parse_my_move(c: &str) -> Move {
    match c {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("unknown my move code \"{}\"", c),
    }
}

fn score_game(data: Vec<Round>) -> i32 {
    data.into_iter()
        .map(|r| r.score())
        .reduce(|a, b| a + b)
        .unwrap()
}

fn load_input_data(path: &str) -> Vec<Round> {
    fs::read_to_string(path)
        .unwrap() // Living dangerously!
        .trim()
        .split("\n")
        .map(|line| {
            let chars: Vec<&str> = line
                .trim()
                .split(" ")
                .collect();

            let a = chars[0];
            let b = chars[1];
            
            Round::new(a, b)

        })
        .collect()
}

fn main() {
    let data = load_input_data(INPUT_FILE);
    println!("number of rounds = {}", data.len());

    let score = score_game(data);
    println!("The score is = {}", score);
}
