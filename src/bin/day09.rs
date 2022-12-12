use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Right(i32),
    Left(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos(i32, i32);


fn main() {
    let raw = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    // Parse the moves...
    let moves: Vec<Move> = raw
        .split("\n")
        .map(|line| {
            // Split the line into a letter and a number...
            let parts: Vec<_> = line.split(" ").collect();
            if parts.len() != 2 {
                panic!("Expected line length 2 - {}", line);
            }

            // Extract the letter and number and parse the number as an int...
            let letter = *parts.get(0).expect("no index 0");
            let count = *parts.get(1).expect("no index 1");
            let count: i32 = count.parse().expect("failed to parse count as an int");

            // Return the move enum based on letter...
            match letter {
                "U" => Move::Up(count),
                "D" => Move::Down(count),
                "L" => Move::Left(count),
                "R" => Move::Right(count),
                _ => unreachable!(),
            }
        })
        .collect();


        // Define starting positions and visited set...
        let mut head = Pos(0, 0);
        let mut tail = Pos(0, 0);
        let mut visited: HashSet<Pos> = HashSet::new();

        // Iterate through the moves...
        for m in moves {
            match m {
                Move::Up(n) => {},
                Move::Down(n) => {},
                Move::Left(n) => {},
                Move::Right(n) => {},
            }
        }

        // Print the number of visited locations...
        println!("I've visited {} places.", visited.len());

}
