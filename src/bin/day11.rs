#[allow(unused_imports)]
use std::fs;
use std::collections::VecDeque;
use regex::Regex;

#[allow(dead_code)]
const INPUT_FILE: &str = "inputs/day-11.txt";


#[allow(dead_code)]
fn parse_monkey_id(line: &str) -> i64 {
    let re = Regex::new(r"Monkey (\d+):")
        .unwrap();
    let cap: &str = re
        .captures(line)
        .expect("failed to get captures")
        .get(1)
        .expect("failed to get capture 1")
        .into();
    cap.parse::<i64>().expect("failed to parse capture as int")
}

#[allow(dead_code)]
fn parse_starting_items(line: &str) -> Vec<i64> {
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
            .parse::<i64>()
            .expect(
                format!("failed to parse {} as a number", s)
                    .as_str()
            )
        )
        .collect()
        ;
    nums
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Value {
    Old,
    Num(i64),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
                .parse::<i64>()
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
fn parse_test(line: &str) -> i64 {
    let re = Regex::new("Test: divisible by ([0-9]+)").unwrap();
    let cap: &str = re.captures(line)
        .expect("no captures found")
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<i64>()
        .expect("failed to parse capture as int")
}

#[allow(dead_code)]
fn parse_test_true(line: &str) -> i64 {
    let re = Regex::new("If true: throw to monkey ([0-9]+)").unwrap();
    let cap: &str = re.captures(line.trim())
        .expect(format!("no captures found for line \"{}\"", line).as_str())
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<i64>()
        .expect("failed to parse capture as int")
    }

#[allow(dead_code)]
fn parse_test_false(line: &str) -> i64 {
    let re = Regex::new("If false: throw to monkey ([0-9]+)").unwrap();
    let cap: &str = re.captures(line)
        .expect("no captures found")
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<i64>()
        .expect("failed to parse capture as int")    
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    id: i64,
    starting_items: Vec<i64>,
    operation: Operation,
    test_div: i64,
    test_true: i64,
    test_false: i64,
}

struct Monkey {
    items: VecDeque<i64>,
    op: Operation,
    test_div: i64,
    test_true: usize,
    test_false: usize,
    count: i64,
}

#[allow(dead_code)]
impl Monkey {
    fn new(items: Vec<i64>, op: Operation, td: i64, tt: usize, tf: usize) -> Self {
        Self {
            items: items.into(),
            count: 0,
            op,
            test_div: td,
            test_true: tt,
            test_false: tf,
        }
    }

    fn add(&mut self, i: i64) {
        self.items.push_back(i);
    }
    
    fn get(&mut self) -> Option<i64> {
        self.items.pop_front()
    }
    
    fn apply(&mut self, i: i64) -> i64 {
        self.count += 1;
        let res = match self.op {
            Operation::Add(v) => {
                match v {
                    Value::Num(n) => i + n,
                    Value::Old => i + i,
                }
            },
            Operation::Mul(v) => {
                match v {
                    Value::Num(n) => i * n,
                    Value::Old => i * i,
                }
            },
        };

        res / 3
    }

    fn test(&self, i: i64) -> usize {
        if i % self.test_div == 0 {
            self.test_true
        } else {
            self.test_false
        }
    }

    fn get_apply_test(&mut self) -> Option<(usize, i64)> {
        // Get the next value from the list...
        let i = self.get()?;

        // Apply the transformation...
        let n = self.apply(i);

        // Deside who to send it to next...
        let to = self.test(n);

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
            test_true: ins.test_true as usize,
            test_false: ins.test_false as usize,
            count: 0,
        }
    }
}

struct State {
    monkeys: Vec<Monkey>,
}

impl State {
    fn new(monkeys: Vec<Monkey>) -> Self {
        State {
            monkeys,
        }
    }

    fn send_to_monkey(&mut self, mi: usize, n: i64) {
        self.monkeys
            .get_mut(mi)
            .expect("that monkey doesn't exist")
            .add(n);
    }

    fn get_apply_test(&mut self, mi: usize) -> Option<(usize, i64)> {
        self.monkeys
            .get_mut(mi)?
            .get_apply_test()
    }

    fn tick(&mut self) {
        for i in 0..self.monkeys.len() {
            // println!("--> Monkey {}", i);
            while let Some((mi, n)) = self.get_apply_test(i) {
                // println!("----> Passing {} to monkey {}", n, mi);
                self.send_to_monkey(mi, n);
            }

        }
        for (i, m) in self.monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", i, m.items.clone());
        }
    }

    fn get_counts(self) -> Vec<i64> {
        self.monkeys
            .iter()
            .map(|m| m.count)
            .collect()
    }
}


fn main() {
    // let raw = fs::read_to_string("inputs/day-11-example.txt").unwrap();
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
    let n_monkeys = monkeys.len();
    
    // Initialize the state... (starting items)
    let mut state = State::new(monkeys);
    
    // Start running the rounds...
    let n_rounds = 20;
    let mut counts: Vec<_> = (0..n_monkeys).map(|i| state.monkeys[i].items.len()).collect();
    for i in 0..n_rounds {
        println!("> Round {}", i);
        state.tick();

        for (i, m) in state.monkeys.iter().enumerate() {
            counts[i] += m.items.len();
        }
    }

    println!();
    println!();
    println!("other counts = {:?}", counts);
    println!();

    let mut counts = state.get_counts();
    println!("counts = {:?}", counts.clone());
    counts.sort();
    counts.reverse();
    

    let c0 = counts[0];
    let c1 = counts[1];
    let score = c0 * c1;
    println!("{} * {} = {}", c0, c1, score);

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey_id() {
        assert_eq!(parse_monkey_id("Monkey 0:"), 0);
        assert_eq!(parse_monkey_id("Monkey 12:"), 12);
    }

    #[test]
    fn test_parse_starting_items() {
        assert_eq!(
            parse_starting_items("Starting items: 12"),
            vec![12],
        );
        assert_eq!(
            parse_starting_items("Starting items: 0"),
            vec![0],
        );
        assert_eq!(
            parse_starting_items("Starting items: 1, 2, 3"),
            vec![1, 2, 3],
        );
        assert_eq!(
            parse_starting_items("Starting items: 100, 200"),
            vec![100, 200],
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("Operation: new = old + 1"),
            Operation::Add(Value::Num(1)),
        );
        assert_eq!(
            parse_operation("  Operation: new = old * 123"),
            Operation::Mul(Value::Num(123)),
        );
        assert_eq!(
            parse_operation("Operation: new = old + old"),
            Operation::Add(Value::Old),
        );
    }

    #[test]
    fn test_parse_test() {
        assert_eq!(parse_test("Test: divisible by 1"), 1);
        assert_eq!(parse_test("Test: divisible by 0"), 0);
        assert_eq!(parse_test("Test: divisible by 123"), 123);
        assert_eq!(parse_test("  Test: divisible by 1  "), 1);
    }

    #[test]
    fn test_parse_test_true() {
        assert_eq!(parse_test_true("If true: throw to monkey 0"), 0);
        assert_eq!(parse_test_true("If true: throw to monkey 1 "), 1);
        assert_eq!(parse_test_true(" If true: throw to monkey 5"), 5);
    }

    #[test]
    fn test_parse_test_false() {
        assert_eq!(parse_test_false("If false: throw to monkey 0"), 0);
        assert_eq!(parse_test_false("If false: throw to monkey 1 "), 1);
        assert_eq!(parse_test_false(" If false: throw to monkey 5"), 5);
        assert_eq!(parse_test_false("    If false: throw to monkey 4"), 4);
    }

}
