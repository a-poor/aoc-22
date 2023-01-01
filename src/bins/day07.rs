use regex::Regex;
use std::collections::{HashMap, HashSet};


const INPUT_PATH: &str = "inputs/day-07.txt";

const TOTAL_DISK_SPACE: usize = 70_000_000;
const DISK_SPACE_NEEDED: usize = 30_000_000;


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

fn get_dir_size(files: &HashMap<String, usize>, prefix: &str) -> usize {
    // Get all files with that as a prefix...
    // - Limit to files with a prefix matching `dir`
    // - Get the sum of all the sizes
    // - If after the filter, the iterator is empty, size is `0`
    files
        .into_iter()
        .filter(|(path, _)| path.starts_with(prefix))
        .map(|(_, size)| *size)
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn main() {
    let raw = std::fs::read_to_string(INPUT_PATH).expect("failed to read input file");

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

    // Find out how much disk space needs to be freed up...
    let space_used = get_dir_size(&files, "/");
    let space_remaining = TOTAL_DISK_SPACE - space_used;
    let space_needed = DISK_SPACE_NEEDED - space_remaining;


    println!("     space_used = {}", space_used);    
    println!("space_remaining = {}", space_remaining);
    println!("   space_needed = {}", space_needed);
    
    let mut dir_sizes: Vec<_> = dirs
        .into_iter()
        .map(|dir| get_dir_size(&files, dir.as_str()))
        .filter(|size| *size > space_needed)
        .collect();
    dir_sizes.sort();
    let smallest = dir_sizes.get(0);
    let largest = dir_sizes.get(dir_sizes.len() - 1);
    
    println!("smallest = {:?}", smallest);
    println!("largest  = {:?}", largest);

}
