use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

const JMP: &str = "jmp";
const ACC: &str = "acc";
const NOP: &str = "nop";

fn main(){
    let vec = parse_file("day8.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let mut visited: Vec<i32> = Vec::new();
    let mut acc: i32 = 0;
    let mut pos: i32 = 0;

    loop {
        let instruction: Vec<String> = input[pos as usize].split(" ").map(|s| s.to_string()).collect();
        let jkl: (&String, i32) = (&instruction[0], instruction[1].parse().expect("Not a number"));
        if visited.contains(&pos) {
            break;
        }
        visited.push(pos);
        if jkl.0 == ACC {
            acc = acc + jkl.1;
            pos = pos + 1;
        }
        else if jkl.0 == JMP {
            pos = pos + jkl.1;
        }
        else {
            pos = pos + 1;
        }
    }
    println!("Part 1: {}", acc);
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let mut flipped:Vec<i32> = Vec::new();
    loop {
        let mut try_replace = true;
        let mut visited: Vec<i32> = Vec::new();
        let mut acc: i32 = 0;
        let mut pos: i32 = 0;
        loop {
            if pos as usize == input.len() {
                println!("Part 2: {}", acc);
                return;
            }
            else if pos as usize > input.len() || pos < 0 {
                break;
            }
            let instruction: Vec<String> = input[pos as usize].split(" ").map(|s| s.to_string()).collect();
            let jkl: (&String, i32) = (&instruction[0], instruction[1].parse().expect("Not a number"));
            if visited.contains(&pos) {
                break;
            }
            visited.push(pos);
            if try_replace && !flipped.contains(&pos){
                if jkl.0 == ACC {
                    acc = acc + jkl.1;
                    pos = pos + 1;
                }
                else if jkl.0 == JMP {
                    flipped.push(pos);
                    try_replace = false;
                    pos = pos + 1;
                }
                else {
                    flipped.push(pos);
                    try_replace = false;
                    pos = pos + jkl.1;
                }
            }
            else {
                if jkl.0 == ACC {
                    acc = acc + jkl.1;
                    pos = pos + 1;
                }
                else if jkl.0 == JMP {
                    pos = pos + jkl.1;
                }
                else {
                    pos = pos + 1;
                }
            }
        }
    }
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