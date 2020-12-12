use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

const DEGREE_INCREMENT: i32 = 90;

fn main(){
    let vec = parse_file("day11.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    // Create vector of strings and numbers. Everybody say "thanks map"
    let orders = input.iter().map(|x| (&x[..1], x[1..].parse::<i32>().unwrap()));

    let mut coordinates = (0,0);
    let mut dir = 0; // 0 is east, 1 is south, 2 is west, 3 is north
    for order in orders {
        //println!("We're at ({}, {}) facing {}. Go {} {}!", coordinates.0, coordinates.1, dir, order.0, order.1);
        match order.0 {
            "N" => coordinates.1 = coordinates.1 + order.1,
            "S" => coordinates.1 = coordinates.1 - order.1,
            "E" => coordinates.0 = coordinates.0 + order.1,
            "W" => coordinates.0 = coordinates.0 - order.1,
            "L" => dir = (dir - (order.1 / DEGREE_INCREMENT)).rem_euclid(4),
            "R" => dir = (dir + (order.1/DEGREE_INCREMENT)).rem_euclid(4),
            "F" => {
                match dir {
                    0 => coordinates.0 = coordinates.0 + order.1,
                    1 => coordinates.1 = coordinates.1 - order.1,
                    2 => coordinates.0 = coordinates.0 - order.1,
                    3 => coordinates.1 = coordinates.1 + order.1,
                    _ => println!("What")
                }
            }
            _ => println!("What")
        }
    }
    //println!("{},{}", coordinates.0, coordinates.1);
    println!("Part 1: {}", i32::abs(coordinates.0) + i32::abs(coordinates.1));
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let orders = input.iter().map(|x| (&x[..1], x[1..].parse::<i32>().unwrap()));
    let mut waypoint_pos = (10,1);
    let mut ship_pos = (0,0);

    // These are how we should mutate for going right or left 90 degrees. Swap the positions and multiply by these
    let left = (-1,1);
    let right = (1,-1);
    for order in orders {
        //println!("We're at ({}, {}) facing {}. Go {} {}!", coordinates.0, coordinates.1, dir, order.0, order.1);
        match order.0 {
            "N" => waypoint_pos.1 = waypoint_pos.1 + order.1,
            "S" => waypoint_pos.1 = waypoint_pos.1 - order.1,
            "E" => waypoint_pos.0 = waypoint_pos.0 + order.1,
            "W" => waypoint_pos.0 = waypoint_pos.0 - order.1,
            "L" => {
                for _x in 0..(order.1/DEGREE_INCREMENT) {
                    waypoint_pos = (waypoint_pos.1 * left.0, waypoint_pos.0*left.1);
                }
            },
            "R" => {
                for _x in 0..(order.1/DEGREE_INCREMENT) {
                    waypoint_pos = (waypoint_pos.1 * right.0, waypoint_pos.0*right.1);
                }
            },
            "F" => {
                ship_pos.0 = ship_pos.0 + (waypoint_pos.0 * order.1);
                ship_pos.1 = ship_pos.1 + (waypoint_pos.1 * order.1);
            },
            _ => println!("What")
        }
    }
    println!("Part 2: {}", i32::abs(ship_pos.0) + i32::abs(ship_pos.1));
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