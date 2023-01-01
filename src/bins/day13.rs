use std::{fmt, fs, collections::VecDeque};
use serde_json::Value;

#[allow(dead_code)]
const INPUT_PATH_REAL: &str = "inputs/day-13.txt";

#[allow(dead_code)]
const INPUT_PATH_EXAMPLE: &str = "inputs/day-13-example.txt";

#[allow(dead_code)]
fn split_lines(raw: &str) -> Vec<(&str, &str)> {
    raw
        .split("\n\n")
        .into_iter()
        .map(|raw_pairs| {
            let mut lines  = raw_pairs.split("\n");
            let left = lines.next().unwrap();
            let right = lines.next().unwrap();
            (left, right)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    NumberVal(i32),
    ArrayVal(Box<Vec<PacketData>>),
}

impl fmt::Display for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketData::NumberVal(n) => f.pad(&format!("{}", n)),
            PacketData::ArrayVal(a) => {
                let mut res = String::new();
                res.push('[');
                for (i, val) in a.iter().enumerate() {
                    if i > 0 {
                        res.push(',');
                    }
                    res.push_str(&format!("{}", val));
                }
                res.push(']');
                f.pad(&format!("{}", res))
            },
        }
    }
}

fn parse_packet_val_from_json(val: &Value) -> PacketData {
    match val {
        Value::Number(n) => PacketData::NumberVal(n.as_i64().unwrap() as i32),
        Value::Array(a) => {
            let res: Vec<PacketData> = a
                .iter()
                .map(|d| parse_packet_val_from_json(d))
                .collect();
            PacketData::ArrayVal(Box::new(res))
        },
        _ => unreachable!("how did you get here? val={:?}", val),
    }
}

fn parse_packet(line: &str) -> Vec<PacketData> {
    let data = serde_json::from_str::<Vec<Value>>(line).unwrap();
    let res: Vec<PacketData> = data.iter().map(parse_packet_val_from_json).collect();
    res
}

/// Checks if two packets are in the right order. Returns None if they're equal.
fn compare_packet_data(left: PacketData, right: PacketData) -> Option<bool> {
    match (left.clone(), right.clone()) {
        // Are they both numbers?
        (PacketData::NumberVal(left), PacketData::NumberVal(right)) => {
            if left == right {
                None
            } else if left < right {
                Some(true)
            } else {
                Some(false)
            }
        },

        // Are both arrays?
        (PacketData::ArrayVal(left), PacketData::ArrayVal(right)) => {
            // Create a deque from the vecs...
            let mut left: VecDeque<_> = left.clone().iter().map(|d| d.clone()).collect();
            let mut right: VecDeque<_> = right.clone().iter().map(|d| d.clone()).collect();

            // Start iterating...
            loop {
                // Get a value from the FRONT of each deque...
                let left_packet = left.pop_front();
                let right_packet = right.pop_front();

                // println!("> Comparing subpackets: {:?} and {:?}", left_packet, right_packet);

                // Compare them...
                match (left_packet, right_packet) {

                    // If they're both numbers, compare them...
                    (Some(left_packet), Some(right_packet)) => {

                        // Was a comparison result returned?
                        // > If so, return it...
                        // > Otherwise, keep going...
                        if let Some(result) = compare_packet_data(left_packet, right_packet) {
                            return Some(result);
                        }
                    },

                    // If they both run out at the same time, check the next one...
                    (None, None) => return None,
                    
                    // If the right list is shorter, they're in the right order...
                    (Some(_), None) => return Some(false),

                    // If the left list is shorter, they're in the wrong order...
                    (None, Some(_)) => return Some(true),
                }
            }
        },

        // Is the left one a number?
        (PacketData::NumberVal(_), PacketData::ArrayVal(_)) => {
            compare_packet_data(
                PacketData::ArrayVal(Box::new(vec![left.clone()])),
                right.clone(),
            )
        },
        
        // Is the right one a number?
        (PacketData::ArrayVal(_), PacketData::NumberVal(_)) => {
            compare_packet_data(
                left.clone(),
                PacketData::ArrayVal(Box::new(vec![right.clone()])),
            )
        },
    }
}

fn compare_packets(left: Vec<PacketData>, right: Vec<PacketData>) -> Option<bool> {
    compare_packet_data(
        PacketData::ArrayVal(Box::new(left)),
        PacketData::ArrayVal(Box::new(right)),
    )
}

fn main() {
    // Read in the source data...
    // let raw = fs::read_to_string(INPUT_PATH_EXAMPLE).unwrap();
    let raw = fs::read_to_string(INPUT_PATH_REAL).unwrap();

    // Split the data into packets and add the divider packets...
    let mut packets = (raw + "\n[[2]]\n[[6]]")
        .split("\n")
        .into_iter()
        .map(|line| {
            let line = line.trim();
            if line == "" { return None }
            Some(parse_packet(line))
        })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect::<Vec<_>>();

    // Sort the packets...
    packets.sort_by(|left, right| {
        if compare_packets(left.clone(), right.clone()).unwrap() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    // Find the divider packets...
    let div2 = vec![PacketData::ArrayVal(Box::new(vec![PacketData::NumberVal(2)]))];
    let div2_pos = packets.iter().position(|p| p == &div2).unwrap() + 1;
    
    let div6 = vec![PacketData::ArrayVal(Box::new(vec![PacketData::NumberVal(6)]))];
    let div6_pos = packets.iter().position(|p| p == &div6).unwrap() + 1;
    println!("Found dividers at positions {} and {}", div2_pos, div6_pos);
    
    // Print the result...
    let total = div2_pos * div6_pos;
    println!("total={:?}", total);
}
