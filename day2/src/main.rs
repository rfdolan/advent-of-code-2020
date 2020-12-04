use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main(){
    std::env::set_var("RUST_BACKTRACE", "1");
    let vec = parse_file("day2.txt");
    //println!("{}", vec[0]);

    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution to part 1
fn part1(vec: &[String]) {
    let mut it = 0;
    for line in vec {
        if is_valid_1(line) {
            it = it + 1;
        }
    }
    println!("Part 1: {}", it);
}

// Solution to part 2
fn part2(vec: &[String]) {
    let mut it = 0;
    for line in vec {
        if is_valid_2(line) {
            it = it + 1;
        }
    }
    println!("Part 2: {}", it);
}

// Split a single line into appropriate values and return a tuple
fn get_vals(password: &str) -> (i32, i32, char, &str){
    let mut split = password.split(" ");
    let numsplit =split.nth(0).unwrap();
    let mut nums = numsplit.split("-");
    let lower: i32 = nums.nth(0).unwrap().parse().expect("Not a number!");
    let upper: i32 = nums.nth(0).unwrap().parse().expect("Not a number!");
    let sought_char: char = split.nth(0).unwrap().chars().next().unwrap();
    let pword = split.nth(0).unwrap();
    return (lower, upper, sought_char, pword);
}

// Check if a given password is valid based on part 1 criteria
fn is_valid_1(password: &str) -> bool {
    let (lower, upper, sought_char, pword) = get_vals(password);

    let mut it = 0;
    for c in pword.chars() {
        if c == sought_char {
            it = it + 1;
        }
    }
    return lower <= it && it <= upper;
}

// Check if a given password is valid based on part 2 criteria
fn is_valid_2(password: &str) -> bool {
    let (lower, upper, sought_char, pword) = get_vals(password);

    let mut found = false;
    for (i, c) in pword.chars().enumerate() {
        if c == sought_char && (i as i32 +1 == lower || i as i32 +1 == upper) {
            if found == true{
                return false;
            }
            found = true;
        }
    }
    
    return found;
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