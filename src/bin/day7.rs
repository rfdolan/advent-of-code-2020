use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashSet;

const SHINY: &str = "shiny";
const GOLD: &str ="gold";

#[derive(Hash, Debug)]
struct Bag {
    pattern: String,
    color: String,
    // Number and type of bag
    contents: Vec<(u32, Bag)>,
}
impl PartialEq for Bag {
    fn eq(&self, other: &Self) ->bool {
        self.pattern == other.pattern && self.color == other.color
    }
}
impl Eq for Bag {}

fn main(){
    let vec = parse_file("day7.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2();
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let mut all: HashSet<Bag> = HashSet::new();
    for bag in input {
        let mut split: Vec<String> = bag.split(" contain ").map(|s| s.to_string()).collect();
        let src= &split[0];
        println!("Src: {}", src);
        let mut parent = create_Bag(&src);

        let mut children = Vec::new();
        let mut kids: Vec<String> = split[1].split(", ").map(|s| s.to_string()).collect();

        for child in kids {
            if child != "no other bags." {
                println!("Child: {}", child);
                let jklsplit: Vec<String> = child.split(" ").map(|s| s.to_string()).collect();
                let num = jklsplit[0].parse().expect("Not a number");
                let mut child_inp = child.clone();
                crop_letters(&mut child_inp, 2);

                let birthed_child = create_Bag(&child_inp);
                children.push((num, birthed_child));
            }
        }
        parent.contents = children;
        all.insert(parent);
    }


    println!("Part 1: {}", 0);
}

fn crop_letters(s: &mut String, pos: usize) {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => {
            s.drain(..pos);
        }
        None => {
            s.clear();
        }
    }
}

fn recurse_lol (bag: &Bag, mut acc: u32, bags: &HashSet<Bag>) -> u32 {
    if bag.color == GOLD && bag.pattern == SHINY {
        return acc + 1;
    }
    /*
    for (num, child) in bag.contents {
        match bags.get(&Bag {color: child.color, pattern: child.pattern, contents: Vec::new()}) {
            None => return acc,
            Some(res) => {
                acc = recurse_lol(res, acc, bags);

            }
        }
            
    }
    */
    return acc;
}


fn create_Bag (bag_str: &String) -> Bag {
    let mut split: Vec<String> = bag_str.split(" ").map(|s| s.to_string()).collect();
    let pattern = &split[0];
    let color = &split[1];
    return Bag {pattern: pattern.to_string(), color: color.to_string(), contents: Vec::new() };
}

// Solution for part 2
fn part2() {
    println!("Part 2: {}", 0);
}

// Parse file with given name in parent directory into a vector of ints
fn parse_file(name: &str) -> Vec<String> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(name) {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip);
            }
        }
    }
    vec
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}