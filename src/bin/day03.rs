use std::vec::Vec;

const TREE_CHAR: char = '#';
// Constants for movement in part 1
const RIGHT: u32 = 3;
const DOWN: u32 = 1;
// Array of movement patterns for part 2
const MOVEMENT_ARR: [(u32, u32); 5] = [(1,1), (3,1), (5,1), (7,1), (1,2)];

fn main(){
    let vec = inp::parse_file("day3.txt");
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(vec: &Vec<String>) {
    let trees = get_num_trees(vec, RIGHT, DOWN);
    println!("Part 1: {}", trees);
}

// Solution for part 2
fn part2(vec: &Vec<String>) {
    let mut ans = 0;
    // Iterate through all the different movement patterns we want to do
    for (right, down) in &MOVEMENT_ARR {
        let trees = get_num_trees(vec, *right, *down);
        // If ans has not been initialized then set it to the first value. 
        // I guess if you get really lucky and hit 0 trees the code will break since your
        // answer would be 0, but whatever nobody's that lucky, right??
        if ans == 0 {
            ans = trees;
        } else {
            ans = ans * trees;
        }
    }
    println!("Part 2: {}", ans);
}

// Get the number of trees going down the given hill in the given movement pattern
fn get_num_trees(vec: &Vec<String>, right: u32, down: u32) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    // Go through every string in our input (aka go down vertically)
    for string in vec {
        // Check if we should care about this one
        if y % down == 0 {
            let bytes = string.as_bytes();
            if bytes[x as usize] as char == TREE_CHAR{
                trees = trees +1;
            }
            x = add_x(x, right, string.len() as u32);
        } 
        y = y+1;
    }
    return trees;
}

// Add to x and wrap
fn add_x(curr: u32, to_add: u32, size: u32) -> u32 {
    let res = curr+to_add;
    if res >= size {
        return res-size;
    }
    return res;
}