use std::vec::Vec;
use regex::Regex;
use std::collections::HashMap;
mod inp;

fn main(){
    let vec = inp::parse_file("day14.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let mut mask = "";
    let mut address_space: HashMap<u32,u64> = HashMap::new();
    for thing in input {
        if thing.contains("mask") {
            mask = thing.split("=").collect::<Vec<&str>>()[1].trim();
        } else {
            let jkl = thing.split("=").collect::<Vec<&str>>();
            let getmem = Regex::new(r"(\d+)").unwrap();
            // putting things on one line like a BOSS
            address_space.insert(getmem.captures(jkl[0]).unwrap()[0].parse::<u32>().unwrap(), mask_num(jkl[1].trim().parse::<u64>().unwrap(), mask.to_string()));
        }
    }
    let mut acc = 0;
    for thing in address_space.iter() {
        acc = thing.1 + acc;
    }
    println!("Part 1: {}", acc);
}

fn mask_num(num: u64, mask: String) -> u64 {
    // Convert to binary and reverse
    let mut num_but_backwards = format!("{:b}", num).chars().rev().collect::<Vec<char>>();
    for (i, x) in mask.chars().rev().enumerate() {
        if i >= num_but_backwards.len() {
            if x == 'X' {
                num_but_backwards.push('0');
            } else {
                num_but_backwards.push(x);
            }
        } else if x != 'X' {
            num_but_backwards[i] = x;
        }
    }
    return calc_num_from_chars(num_but_backwards);
}

// epic function that turns a backwards array of bits into a number
fn calc_num_from_chars(chars: Vec<char>) -> u64 {
    let mut acc = 0;
    let mut fac = 1;
    for num in chars {
        if num == '1' {
            acc = acc + fac;
        }
        fac = fac * 2;
    }
    return acc;
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let mut mask = "";
    let mut address_space: HashMap<u64,u64> = HashMap::new();
    for thing in input {
        if thing.contains("mask") {
            mask = thing.split("=").collect::<Vec<&str>>()[1].trim();
        } else {
            // Putting things on multiple lines like a BOSS
            let jkl = thing.split("=").collect::<Vec<&str>>();
            let getmem = Regex::new(r"(\d+)").unwrap();
            let val = jkl[1].trim().parse::<u64>().unwrap();
            let mem = mask_num_2(getmem.captures(jkl[0]).unwrap()[0].parse::<u32>().unwrap(), mask.to_string());
            for addr in mem {
                address_space.insert(addr, val);
            }
        }
    }

    let mut acc: u128 = 0;
    for thing in address_space.iter() {
        acc = *thing.1 as u128 + acc;
    }
    println!("Part 2: {}", acc);
}

fn mask_num_2(num: u32, mask: String) -> Vec<u64> {
    // Convert to binary and reverse
    let mut num_but_backwards = format!("{:b}", num).chars().rev().collect::<Vec<char>>();
    for (i, x) in mask.chars().rev().enumerate() {
        if i >= num_but_backwards.len() {
            num_but_backwards.push(x);
        } else if x != '0' {
            num_but_backwards[i] = x;
        }
    }

    // Get vector of all possible addresses in base 2 form
    let vec: Vec<String> = Vec::new();
    let all = get_all_possible( vec, num_but_backwards.iter().collect::<String>());
    let mut res = Vec::new();

    // Create and return vec of all possible addresses in base 10
    for x in all {
        res.push(calc_num_from_chars(x.chars().rev().collect::<Vec<char>>()));
    }
    return res;
}

// Recursive function to get all possible versions of a memory string
fn get_all_possible(array: Vec<String>, string: String) -> Vec<String> {
    // Base case, add to vec and return
    if !string.contains('X') {
        let mut pass = array.clone();
        pass.push(string);
        return pass;
    } else {
        // Call with both possible versions
        // Also do some array trickery because rust gets mad at me and idk how to do it well
        let pass = array.clone();
        let mut res = get_all_possible(array, string.replacen("X", "0",1)).clone();
        let mut res2 = get_all_possible(pass,string.replacen("X", "1",1)).clone();
        res.append(&mut res2);
        let aaa: Vec<String> = res.iter().cloned().collect::<Vec<String>>();
        return aaa;
    }
}