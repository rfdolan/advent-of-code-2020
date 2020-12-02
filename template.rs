use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main(){
    let vec = parse_file("xxx.txt");
    // Put the code to do the thing here
    part1();
    part2();
}

// Solution for part 1
fn part1() {
    println!("Part 1: {}", 0);
}

// Solution for part 2
fn part2() {
    println!("Part 2: {}", 0);
}

// Parse file with given name in parent directory into a vector of ints
fn parse_file(name: &str) -> Vec<i32> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(name) {
        for line in lines {
            if let Ok(ip) = line {
                let line_num: i32 = ip.parse().expect("Not a number!");
                vec.push(line_num);
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