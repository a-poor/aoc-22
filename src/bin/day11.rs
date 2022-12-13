#[allow(unused_imports)]
use std::{fs, collections::HashMap};
use regex::Regex;

#[allow(dead_code)]
const INPUT_FILE: &str = "inputs/day-11.txt";


#[allow(dead_code)]
fn parse_monkey_id(line: &str) -> i32 {
    let re = Regex::new(r"Monkey (\d+):")
        .unwrap();
    let cap: &str = re
        .captures(line)
        .expect("failed to get captures")
        .get(1)
        .expect("failed to get capture 1")
        .into();
    cap.parse::<i32>().expect("failed to parse capture as int")
}

#[allow(dead_code)]
fn parse_starting_items(line: &str) -> Vec<i32> {
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
            .parse::<i32>()
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
    Num(i32),
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
                .parse::<i32>()
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
fn parse_test(line: &str) -> i32 {
    let re = Regex::new("Test: divisible by ([0-9]+)").unwrap();
    let cap: &str = re.captures(line)
        .expect("no captures found")
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<i32>()
        .expect("failed to parse capture as int")
}

#[allow(dead_code)]
fn parse_test_true(line: &str) -> i32 {
    let re = Regex::new("If true: throw to monkey ([0-9]+)").unwrap();
    let cap: &str = re.captures(line)
        .expect("no captures found")
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<i32>()
        .expect("failed to parse capture as int")
    }

#[allow(dead_code)]
fn parse_test_false(line: &str) -> i32 {
    let re = Regex::new("If false: throw to monkey ([0-9]+)").unwrap();
    let cap: &str = re.captures(line)
        .expect("no captures found")
        .get(1)
        .expect("failed to get regex capture")
        .into();
    cap.parse::<i32>()
        .expect("failed to parse capture as int")    
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    id: i32,
    operation: Operation,
    test_div: i32,
    test_true: i32,
    test_false: i32,
}


fn main() {
    let raw = fs::read_to_string(INPUT_FILE).unwrap();
    let chunks: Vec<_> = raw
        .split("\n\n")
        .map(|chunk| {
            let lines: Vec<_> = chunk.split("\n").collect();
            let ins = Instruction {
                id: parse_monkey_id(lines[0]),
                operation: parse_operation(lines[2]),
                test_div: parse_test(lines[3]),
                test_true: parse_test_true(lines[4]),
                test_false: parse_test_true(lines[5]),
            };
            let start = parse_starting_items(lines[1]);
            (ins, start)
        })
        .collect();
    let monkeys: Vec<_> = chunks
        .clone()
        .into_iter()
        .map(|(m, _)| m)
        .collect();
    
    // Initialize the state... (starting items)
    let mut state: HashMap<_, _> = chunks
        .into_iter()
        .map(|(m, start)| (m.id, start))
        .collect();

    let n_rounds = 20;

    // Start running the rounds...
    for _ in 0..n_rounds {

    }
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
    }

}
