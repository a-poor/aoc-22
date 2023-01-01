use std::{cmp::Ordering, fs};

const INPUT_FILE: &str = "inputs/day-02.txt";

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
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

#[derive(Clone)]
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
    their_move: Move,
    goal: Outcome,
}

impl Round {
    fn new(them: &str, goal: &str) -> Self {
        Round {
            their_move: parse_their_move(them),
            goal: parse_goal(goal),
        }
    }

    fn my_move(&self) -> Move {
        match (self.their_move.clone(), self.goal.clone()) {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Tie) => Move::Rock,
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Tie) => Move::Paper,
            (Move::Paper, Outcome::Loss) => Move::Rock,
    
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Tie) => Move::Scissors,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
        }
    }

    fn score(&self) -> i32 {
        self.goal.score() + self.my_move().score()
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

fn parse_goal(c: &str) -> Outcome {
    match c {
        "X" => Outcome::Loss,
        "Y" => Outcome::Tie,
        "Z" => Outcome::Win,
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
