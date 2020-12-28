use std::vec::Vec;
use regex::Regex;
use std::collections::HashMap;

const SHINY_GOLD: &str = "shiny gold";

fn main(){
    let map = create_map(&inp::parse_file("day7.txt"));
    // Put the code to do the thing here
    part1(&map);
    part2(&map);
}

// I hate this but eh
fn create_map(input: &Vec<String>) -> HashMap<String, Vec<(i32, String)>> {
    let mut map: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    let re_first_two = Regex::new(r"[^\s]+\s+[^\s]+").unwrap();
    for string in input {
        let split = string.split("contain ").collect::<Vec<&str>>();
        let buh = re_first_two.captures(split[0]).unwrap();
        let key = buh[0].to_string();
        let mut children = Vec::new();
        if !split[1].contains("no other bags") {
            let contained = split[1].split(", ").collect::<Vec<&str>>();
            for contained_bag in contained {
                let contained_split = contained_bag.split(" ").collect::<Vec<&str>>();
                let quant: i32 = contained_split[0].parse().unwrap();
                let mut bag_type = String::from(""); // too lazy to learn regex
                bag_type.push_str(contained_split[1]);
                bag_type.push_str(" ");
                bag_type.push_str(contained_split[2]);
                children.push((quant, bag_type));
            }
        }
        map.insert(key, children);
    }
    map
}

// Solution for part 1
fn part1(map: &HashMap<String, Vec<(i32, String)>>) {
    let mut contains_target: Vec<String> = Vec::new(); // keep track of ones that contain shiny gold so we dont have to go all the way every time
    for x in map.iter() {
        for sub in x.1 {
            if contains_target_bag(sub.1.clone(), map) {
                contains_target.push(x.0.clone());
                break;
            }
        }
    }
    println!("Part 1: {}", contains_target.len());
}

fn contains_target_bag(curr: String, map: &HashMap<String, Vec<(i32, String)>>) -> bool{
    if curr == SHINY_GOLD {
        return true;
    } 
    let children = map.get(&curr).unwrap();
    return children.iter().fold(false, |acc, x| acc || contains_target_bag(x.1.clone(), map)); // fold gang
}

// Solution for part 2
fn part2(map: &HashMap<String, Vec<(i32, String)>>) {
    println!("Part 2: {}", num_bags_in(String::from(SHINY_GOLD), map)-1); // subtract 1 because we dont contain ourselves
}

fn num_bags_in(curr: String, map: &HashMap<String, Vec<(i32, String)>>) -> i64 {
    let mut acc = 1;
    let target = map.get(&curr).unwrap();
    for x in target {
        acc = acc + (x.0 as i64 * num_bags_in(x.1.clone(), map));
    }
    acc
}