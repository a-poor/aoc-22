#[allow(unused_imports)]
use std::fs;
use std::collections::VecDeque;
use regex::Regex;

#[allow(dead_code)]
const INPUT_FILE: &str = "inputs/day-11.txt";


#[allow(dead_code)]
fn parse_monkey_id(line: &str) -> usize {
    let re = Regex::new(r"Monkey (\d+):")
        .unwrap();
    let cap: &str = re
        .captures(line)
        .expect("failed to get captures")
        .get(1)
        .expect("failed to get capture 1")
        .into();
    cap.parse::<_>().expect("failed to parse capture as int")
}

#[allow(dead_code)]
fn parse_starting_items(line: &str) -> Vec<u128> {
    let re = Regex::new(r"Starting items: ([0-9]+(, [0-9]+)*)")
        .unwrap();
    let cap: &str = re
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .into();

    let nums: Vec<_> = cap
        .split(", ")
        .map(|s| s
            .parse::<u128>()
            .expect(
                format!("failed to parse {} as a number", s)
                    .as_str()
            )
        )
        .collect()
        ;
    nums
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Old,
    Num(u128),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Add(Value),
    Mul(Value),
}

#[allow(dead_code)]
fn parse_operation(line: &str) -> Operation {
    // Create and apply the regex...
    let re = Regex::new(
        r"Operation: new = old ([+*]) ([0-9]+|old)"
    ).unwrap();
    let caps = re
        .captures(line)
        .expect("failed to parse captures");
    
    // Extract the number...
    let num_cap: &str = caps
        .get(2)
        .expect("failed to capture number")
        .into();
    let val = match num_cap {
        "old" => Value::Old,
        _ => {
            let n = num_cap
                .parse::<u128>()
                .expect("failed to parse num as int");
            Value::Num(n)
        },
    };

    // Extract the operation...
    let op_cap: &str = caps
        .get(1)
        .expect("failed to capture operation")
        .into();
    match op_cap {
        "+" => Operation::Add(val),
        "*" => Operation::Mul(val),
        _ => unreachable!("operation was \"{}\"", op_cap),
    }
}

#[allow(dead_code)]
fn parse_test(line: &str) -> u128 {
    let re = Regex::new("Test: divisible by ([0-9]+)").unwrap();
    let cap: &str = re.captures(line)
        .expect("no captures found")
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<u128>()
        .expect("failed to parse capture as int")
}

#[allow(dead_code)]
fn parse_test_true(line: &str) -> usize {
    let re = Regex::new("If true: throw to monkey ([0-9]+)").unwrap();
    let cap: &str = re.captures(line.trim())
        .expect(format!("no captures found for line \"{}\"", line).as_str())
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<_>()
        .expect("failed to parse capture as int")
    }

#[allow(dead_code)]
fn parse_test_false(line: &str) -> usize {
    let re = Regex::new("If false: throw to monkey ([0-9]+)").unwrap();
    let cap: &str = re.captures(line)
        .expect("no captures found")
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<_>()
        .expect("failed to parse capture as int")    
}

fn gcd(a: u128, b: u128) -> u128 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a, b)
}

fn find_lcm(nums: Vec<u128>) -> Option<u128> {
    nums
        .into_iter()
        .reduce(|a, b| lcm(a, b))
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    id: usize,
    starting_items: Vec<u128>,
    operation: Operation,
    test_div: u128,
    test_true: usize,
    test_false: usize,
}

struct Monkey {
    items: VecDeque<u128>,
    op: Operation,
    test_div: u128,
    test_true: usize,
    test_false: usize,
    count: u128,
}

#[allow(dead_code)]
impl Monkey {
    fn new(items: Vec<u128>, op: Operation, td: u128, tt: usize, tf: usize) -> Self {
        Self {
            items: items.into(),
            count: 0,
            op,
            test_div: td,
            test_true: tt,
            test_false: tf,
        }
    }

    fn add(&mut self, i: u128) {
        self.items.push_back(i);
    }
    
    fn get(&mut self) -> Option<u128> {
        self.items.pop_front()
    }
    
    fn apply(&mut self, i: u128, lcm: u128) -> u128 {
        self.count += 1;
        let res = match self.op.clone() {
            Operation::Add(v) => {
                match v {
                    Value::Num(n) => i + n,
                    Value::Old => i.clone() + i.clone(),
                }
            },
            Operation::Mul(v) => {
                match v {
                    Value::Num(n) => i * n,
                    Value::Old => i.clone() * i.clone(),
                }
            },
        };

        res % lcm
    }

    fn test(&self, i: u128) -> usize {
        if i % self.test_div.clone() == 0 {
            self.test_true
        } else {
            self.test_false
        }
    }

    fn get_apply_test(&mut self, lcm: u128) -> Option<(usize, u128)> {
        // Get the next value from the list...
        let i = self.get()?;

        // Apply the transformation...
        let n = self.apply(i, lcm);

        // Deside who to send it to next...
        let to = self.test(n.clone());

        // Return the monkey to send it to 
        // and the value to send...
        Some((to, n))
    }
}

impl From<Instruction> for Monkey {
    fn from(ins: Instruction) -> Monkey {
        Monkey {
            items: ins.starting_items.into(),
            op: ins.operation,
            test_div: ins.test_div,
            test_true: ins.test_true,
            test_false: ins.test_false,
            count: 0,
        }
    }
}

struct State {
    monkeys: Vec<Monkey>,
    mlcm: u128,
}

impl State {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let divs: Vec<_> = monkeys
            .iter()
            .map(|m| m.test_div)
            .collect();
        let mlcm = find_lcm(divs)
            .expect("couldn't find an lcm");
        State {
            monkeys,
            mlcm,
        }
    }

    fn send_to_monkey(&mut self, mi: usize, n: u128) {
        self.monkeys
            .get_mut(mi)
            .expect("that monkey doesn't exist")
            .add(n);
    }

    fn get_apply_test(&mut self, mi: usize) -> Option<(usize, u128)> {
        self.monkeys
            .get_mut(mi)?
            .get_apply_test(self.mlcm)
    }

    fn tick(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some((mi, n)) = self.get_apply_test(i) {
                self.send_to_monkey(mi, n);
            }

        }
    }

    fn get_counts(self) -> Vec<u128> {
        self.monkeys
            .iter()
            .map(|m| m.count)
            .collect()
    }
}


fn main() {
    let raw = fs::read_to_string(INPUT_FILE).unwrap();
    let monkeys: Vec<Monkey> = raw
        .split("\n\n")
        .map(|chunk| {
            let lines: Vec<_> = chunk
                .split("\n")
                .map(|line| line.trim())
                .collect();
            Instruction {
                id: parse_monkey_id(lines[0]),
                starting_items: parse_starting_items(lines[1]),
                operation: parse_operation(lines[2]),
                test_div: parse_test(lines[3]),
                test_true: parse_test_true(lines[4]),
                test_false: parse_test_false(lines[5]),
            }.into()
        })
        .collect();
    
    // Initialize the state... (starting items)
    let mut state = State::new(monkeys);
    
    // Start running the rounds...
    let n_rounds = 10_000;
    for _ in 0..n_rounds {
        state.tick();
    }

    let mut counts = state.get_counts();
    println!("counts = {:?}", counts.clone());
    counts.sort();
    counts.reverse();
    

    let c0 = counts[0];
    let c1 = counts[1];
    let score = c0 * c1;
    println!("{} * {} = {}", c0, c1, score);

}

