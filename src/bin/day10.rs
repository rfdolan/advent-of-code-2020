
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main(){
    let vec = parse_file("day10.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(input: &Vec<i64>) {

    let max = *input.iter().max().unwrap() + 3;
    let mut curr = 0;
    let mut num1 = 0;
    let mut num3 = 0;
    while curr != max {
        match input.iter().find(|&&x| x==curr+1) {
            Some(num) => {
                curr = curr + 1;
                num1 = num1 + 1;
            }
            None => {
                curr = curr + 3;
                num3 = num3 + 1;
            }
        }
    }
   
    println!("Part 1: {}", num1*num3);
}

// Solution for part 2
fn part2(input: &Vec<i64>) {
    let mut vec = input.clone();
    vec.sort();
    /*
    vec.reverse();
    vec.push(0);
    vec.reverse();
    vec.push(vec[vec.len()-1]);
    */
    let mut mult: i64 = 1;

    for (i, num) in vec.iter().enumerate() {
        match vec.iter().position(|&x| x == num + 3) {
            Some(n) => {
                let dif = n - i;
                match dif {
                    3 => {
                        println!("Sequence of 4 found from {} to {}",i,n);
                        if mult == 1 {
                            mult =  5;
                        } else {
                            mult = mult * 3;
                        }
                    },
                    2 => {
                        println!("Sequence of 3 found from {} to {}",i,n);
                        if mult == 1 {
                            mult = 2;
                        }else {
                            mult = mult * 2;
                        }

                    }
                    _ => continue,
                }
            },
            None => {
                match vec.iter().find(|&&x1| x1==num-1) {
                    Some(_) => continue,
                    None =>{
                        match vec.iter().position(|&x2| x2==num+2) {
                            Some(n1) => {
                                let dif1 = n1 - i;
                                match dif1 {
                                    2 =>{
                                        println!("Sequence of 3 found from {} to {}",i,n1);
                                        if mult == 1 {
                                            mult = 2;
                                        }else {
                                            mult = mult * 2;
                                        }
                                    }, 
                                    _ => continue,
                                }
                            }
                            None => continue
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", mult);
}

// Parse file with given name in parent directory into a vector of ints
fn parse_file(name: &str) -> Vec<i64> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(name) {
        for line in lines {
            if let Ok(ip) = line {
                let line_num: i64 = ip.parse().expect("Not a number!");
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