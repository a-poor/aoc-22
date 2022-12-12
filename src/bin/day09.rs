use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move (Direction, i32);


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(i32, i32);


fn direction(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n / n.abs()
    }
}


fn move_head(p: Pos, d: Direction) -> Pos {
    // Extract the x and y coordinates from the head's position
    let Pos(x, y) = p;

    // Move accordingly...
    match d {
        Direction::Up => Pos(x, y-1),
        Direction::Down => Pos(x, y+1),
        Direction::Left => Pos(x-1, y),
        Direction::Right => Pos(x+1, y),
    }
}

fn move_tail(head: Pos, tail: Pos) -> Pos {
    // Extract the x and y coords for head and tail...
    let Pos(hx, hy) = head;
    let Pos(tx, ty) = tail;

    // Calculate the distances / deltas...
    let dx = hx - tx;
    let dy = hy - ty;

    // Calculate the absolute values of the distances...
    let adx = dx.abs();
    let ady = dy.abs();

    // If the tail's distance is w/in one square (including diagonals), do nothing...
    if adx <= 1 && ady <= 1 {
        return Pos(tx, ty);
    }

    // Calculate the directions to move...
    // 
    // The `direction` function will return +1 or -1 depending on the sign of the
    // value passed in (unless it's 0, in which case `direction` will return 0).
    // 
    // If the tail is in the same column or row as the head, the move amount
    // will be zero for that direction. For diagonal differences, there will
    // be a move in both directions. 
    let mx = direction(dx);
    let my = direction(dy);
    
    // Return the position with the moves applied...
    Pos(tx+mx, ty+my)

}


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
                "U" => Move(Direction::Up, count),
                "D" => Move(Direction::Down, count),
                "L" => Move(Direction::Left, count),
                "R" => Move(Direction::Right, count),
                _ => unreachable!(),
            }
        })
        .collect();


        // Define starting positions and visited set...
        let mut head = Pos(0, 0);
        let mut tail = Pos(0, 0);
        let mut visited: HashSet<Pos> = HashSet::new();
        
        // Add the tail's starting position...
        visited.insert(tail);

        // Iterate through the moves...
        for m in moves {
            // Split the move into direction and number of moves...
            let Move(d, n) = m;

            // Iterate through the count, applying each move...
            for _ in 0..n {
                // Move the head...
                head = move_head(head, d);

                // Move the tail...
                tail = move_tail(head, tail);

                // Add the tail's position to the visited set...
                visited.insert(tail);
            }
        }

        // Print the number of visited locations...
        println!("I've visited {} places.", visited.len());

}
