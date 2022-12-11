use regex::Regex;

// struct File {
//     parent: String,
//     name: String,
//     size: i32,
// }

// struct Directory {
//     parent: Option<String>,
//     name: String,
// }

// struct FileSystem {
//     dirs: Vec<Directory>,
//     files: Vec<File>,
// }

// impl FileSystem {
//     fn new() -> Self {
//         FileSystem { 
//             dirs: Vec::new(), 
//             files: Vec::new(), 
//         }
//     }
// }

// use std::collections::HashMap;

// struct File {
//     dir: String,
//     name: String,
//     size: i32,
// }

#[derive(Debug, PartialEq, Eq)]
enum Line {
    ListDir,
    ChDir(String),
    DirInDir(String),
    FileInDir(usize, String),
}

fn parse_line(line: &str) -> Line {
    // List directory?
    if line == r"$ ls" {
        return Line::ListDir;
    }

    // Change directories?
    let re = Regex::new(r"^\$ cd (.+)$").unwrap();
    if re.is_match(line) {
        let d = re.captures(line).expect("no matches");
        let dir: &str = d.get(1).expect("no 0th match").into();
        return Line::ChDir(dir.into());
    }

    // Listed directory?
    let re = Regex::new("dir (.+)").unwrap();
    if re.is_match(line) {
        let d = re.captures(line).expect("no matches");
        let dir: &str = d.get(1).expect("no 0th match").into();
        return Line::DirInDir(dir.into());
    }

    // Listed file?
    let re = Regex::new("^([0-9]+) (.+)$").unwrap();
    if re.is_match(line) {
        let d = re.captures(line).expect("no matches");
        let size: &str = d.get(1).expect("no 0th match").into();
        let size: usize = size.parse().expect(format!("not a number line=\"{}\", size=\"{}\"", line, size).as_str());
        let name: &str = d.get(2).expect("no 1st match").into();
        return Line::FileInDir(size, name.into());
    }

    panic!("Oh no! How did I get here? Line: \"{}\"", line);
}

fn main() {
    let raw = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let _lines: Vec<_> = raw
        .split("\n")
        .map(|line| parse_line(line))
        .collect()
        ;

    // for line in lines {
    //     match line {
    //         Line::ListDir => println!("> Listing directories"),
    //         Line::ChDir(path) => println!("> Changing directories to: {}", path),
    //         Line::DirInDir(dir) => println!("? Found a dir: {}", dir),
    //         Line::FileInDir(size, name) => println!("> Found a file: {} ({} bytes)", name, size),
    //     }
    // }

    // let mut files: HashMap<String, File> = HashMap::new();
    // let mut cdir: Vec<&str> = Vec::new();

    assert_eq!(Line::ChDir("/".into()), parse_line(r"$ cd /"));
    assert_eq!(Line::ChDir("my_stuff".into()), parse_line("$ cd my_stuff"));
    assert_eq!(Line::ChDir("..".into()), parse_line("$ cd .."));
    assert_eq!(Line::ListDir, parse_line("$ ls"));
    assert_eq!(Line::DirInDir("my_stuff".into()), parse_line("dir my_stuff"));
    assert_eq!(Line::FileInDir(12345, "demo.txt".into()), parse_line("12345 demo.txt"));


}
