use regex::Regex;
use std::collections::{HashMap, HashSet};


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

fn strip_last_path(p: &str) -> String {
    if !p.contains("/") {
        "".into()
    } else {
        let re = Regex::new(r"/[^/]+$").unwrap();
        re.replace(p, "").into()
    }
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

    let lines: Vec<_> = raw
        .split("\n")
        .map(|line| parse_line(line))
        .collect()
        ;

    let mut files: HashMap<String, usize> = HashMap::new();
    let mut dirs: HashSet<String> = HashSet::new();
    let mut cdir: String = "".into();

    for line in lines {
        match line {
            Line::ListDir => { /* noop */ },
            Line::ChDir(dir) => {
                if dir == "/" { // Move to root?
                    cdir = "".into();
                
                } else if dir == ".." { // Move up one?
                    cdir = strip_last_path(cdir.as_str());

                } else { // Move down into dir?
                    cdir = format!("{}/{}", cdir, dir);
                
                }
            },
            Line::DirInDir(_dir) => { /* noop for now */ },
            Line::FileInDir(size, name) => {
                let path = format!("{}/{}", cdir, name);
                files.insert(path, size);
            },
        }
        if cdir.starts_with("/") {
            dirs.insert(format!("{}", cdir));
        } else {
            dirs.insert(format!("/{}", cdir));
        }
    }

    // println!("files = {:?}", files);
    // println!("dirs = {:?}", dirs);
    
    let max_size: usize = 100_000;

    let total = dirs
        .into_iter()
        .map(|dir| {
            // Get all files with that as a prefix...
            // - Limit to files with a prefix matching `dir`
            // - Get the sum of all the sizes
            // - If after the filter, the iterator is empty, size is `0`
            files
                .clone()
                .into_iter()
                .filter(|(path, _)| path.starts_with(&dir))
                .map(|(_, size)| size)
                .reduce(|a, b| a + b)
                .unwrap_or(0)
        })
        .filter(|size| *size <= max_size)
        .reduce(|a, b| a + b);
    
        println!("total = {:?}", total);
    

}
