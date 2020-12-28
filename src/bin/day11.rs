use std::vec::Vec;

const DIFFS: [(i32,i32); 8] = [(0,1), (0,-1), (1,0), (1,1), (1,-1), (-1,0),(-1,-1),(-1,1)];
const NUM_TO_VACATE_1: u32 = 4;
const NUM_TO_VACATE_2: u32 = 5;
const OCCUPIED_SEAT: char = '#';
const UNOCCUPIED_SEAT: char = 'L';
const FLOOR: char = '.';
fn main(){
    let vec = inp::parse_file("day11.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Check if two boards are the same
fn unchanged(prev: &Vec<Vec<char>>, curr: &Vec<Vec<char>>) -> bool {
    for y in 0..prev.len() {
        for x in 0..prev[0].len() {
            if prev[y][x] != curr[y][x] {
                return false;
            }
        }
    }
    return true;
}
/*
fn print_board(board: &Vec<Vec<char>>) {
    for x in board {
        for y in x {
            print!("{}", y);
        }
        println!("");
    }
}
*/

// Get total number of occupied seats given board
fn get_num_occupied(board: &Vec<Vec<char>>) -> u32 {
    let mut acc = 0;
    for x in board{
        for y in x {
            if *y == OCCUPIED_SEAT{
                acc = acc + 1;
            }
        }
    }
    acc
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let ysize = input.len();
    let xsize = input[0].len();
    let mut board: Vec<Vec<char>> = vec![vec!['.'; xsize]; ysize];
    // populate the board
    for (y, string) in input.iter().enumerate() {
        for (x, character) in string.chars().collect::<Vec<char>>().iter().enumerate() {
            board[y][x] = *character;
        }
    }
    loop  {
        let prev = board.clone();
        // do iteration, changing the changed variable if a seat becomes occupied.
        board = do_step(board);
        if unchanged(&board, &prev) { break;}
    } 
    println!("Part 1: {}", get_num_occupied(&board));
}

// Do step given part 1 rules
fn do_step(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let old_state = board.clone();
    let mut new = board.clone();
    for (y, string) in board.iter().enumerate() {
        for (x, space) in string.iter().enumerate() {
            // make seat empty
            if *space != '.' {
                if  get_occupied_neighbors(&old_state, (x,y)) >= NUM_TO_VACATE_1{
                    new[y][x] = UNOCCUPIED_SEAT;
                } else if get_occupied_neighbors(&old_state, (x,y)) == 0{
                    new[y][x] = OCCUPIED_SEAT;
                }
            } 
        }
    }
    new
}

// Part 1 find occupied surrounding seats
fn get_occupied_neighbors(board: &Vec<Vec<char>>, pos: (usize, usize)) -> u32{
    let mut num_occ = 0;
    let (x,y) = pos;
    let xsize = board[0].len();
    let ysize = board.len();
    for dif in DIFFS.iter() {
        let xdif = x as i32 + dif.0;
        let ydif = y as i32 + dif.1;
        if ydif >= 0 && ydif < ysize as i32 && xdif >= 0 && xdif < xsize as i32 {
            let other = board[ydif as usize][xdif as usize];
            if other == OCCUPIED_SEAT{
                num_occ = num_occ + 1;
            }
        }
    }
    num_occ
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let ysize = input.len();
    let xsize = input[0].len();
    let mut board: Vec<Vec<char>> = vec![vec![FLOOR; xsize]; ysize];
    // populate the board
    for (y, string) in input.iter().enumerate() {
        for (x, character) in string.chars().collect::<Vec<char>>().iter().enumerate() {
            board[y][x] = *character;
        }
    }
    loop  {
        // do iteration, changing the changed variable if a seat becomes occupied.
        let prev = board.clone();
        board = do_step_2(board);
        if unchanged(&board, &prev) { break;}
    } 
    println!("Part 2: {}", get_num_occupied(&board));
}

// Do iteration given part 2 rules
fn do_step_2(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let old_state = board.clone();
    let mut new = board.clone();
    for (y, string) in board.iter().enumerate() {
        for (x, space) in string.iter().enumerate() {
            // make seat empty or occupied
            if *space != FLOOR{
                let in_sight = get_occupied_in_sight(&old_state, (x,y));
                if  in_sight >= NUM_TO_VACATE_2{
                    new[y][x] = UNOCCUPIED_SEAT;
                } else if in_sight == 0{
                    new[y][x] = OCCUPIED_SEAT;
                }
            } 
        }
    }
    new
}

// Get occupied seats in sight
fn get_occupied_in_sight(board: &Vec<Vec<char>>, pos: (usize, usize)) -> u32{
    let mut num_occ = 0;
    let (x,y) = pos;
    let xsize = board[0].len();
    let ysize = board.len();
    for dif in DIFFS.iter() {
        let mut xdif = x as i32;
        let mut ydif = y as i32;
        // while in range and havent seen a seat
        while ydif + dif.1 >= 0 && ydif + dif.1 < ysize as i32 && xdif +dif.0 >= 0 && xdif + dif.0 < xsize as i32 {
            ydif = ydif + dif.1;
            xdif = xdif + dif.0;

            let other = board[ydif as usize][xdif as usize];
            if other == OCCUPIED_SEAT{
                num_occ = num_occ + 1;
                break;
            } else if other == UNOCCUPIED_SEAT{
                break;
            }
        }
    }
    num_occ
}