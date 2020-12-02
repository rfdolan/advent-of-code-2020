use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let vec = parse_file("day1.txt");
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(vec: &[i32]) {
    for int1 in vec.iter() {
        let target = 2020 - int1;
        if vec.contains(&target) {
            println!("Part 1: {}", int1*target);
            return;
        }
    }
}

// Solution for part 2
fn part2(vec: &[i32]) {
    for int1 in vec.iter() {
        for int2 in vec.iter() {
            let target = 2020 - int1 -int2;
            if vec.contains(&target) {
                println!("Part 2: {}", int1*int2*target);
                return;
            }
        }
    }
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

// Read each line of a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
