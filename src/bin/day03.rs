use std::collections::HashSet;

// struct Rucksack {
//     left: String,
//     right: String,
// }
// impl Rucksack {}


fn find_repeat(line: &str) -> Result<char, String> {
    // Validate the line length...
    let line_len = line.len();
    if line_len == 0 {
        return Err("can't split zero-length line".into());
    }
    if line_len % 2 == 1 {
        return Err(format!("invalid line length. line length is odd ({})", line_len));
    }

    // Split into left and right...
    let mid = line_len / 2;
    let left = &line[..mid];
    let right = &line[mid..];

    // Convert them into sets...
    let left_set: HashSet<char> = left.chars().collect();
    let right_set: HashSet<char> = right.chars().collect();

    // Check the intersection...
    let intersect: Vec<&char> = left_set.intersection(&right_set).collect();

    // Validate the intersection...
    let intersect_len = intersect.len();
    if intersect_len == 0 {
        return Err("\"left ∩ right\" is empty".into());
    }
    if intersect_len > 1 {
        return Err(format!("\"left ∩ right\" is greater than 1 ({})", intersect_len));
    }

    let dupe = intersect[0];
    Ok(*dupe)
}


fn priority(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        c as i32 - 'a' as i32 + 1
    } else if c >= 'A' && c <= 'Z' {
        c as i32 - 'A' as i32 + 27
    } else {
        panic!("can't prioritize unknown character '{}'", c);
    }
}


fn main() {
    let raw = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let total = raw
        .split("\n") // Split into lines...
        .map(|line| {
            find_repeat(line)
                .expect("line repeat find failed")
        }) // Find the duplicate characters...
        .map(|c| priority(c))
        .reduce(|a, b| a + b)
        .expect("no lines")
        ;
    println!("Result = {}", total);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_test() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('M'), 39);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    #[should_panic(expected = "can't prioritize unknown character '1'")]
    fn priority_test_panic() {
        priority('1');
    }
}
