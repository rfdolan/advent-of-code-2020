use std::vec::Vec;
use std::collections::HashMap;

const SURROUNDING :[(i32,i32,i32);26] = generate_surroundings_3d();
const SURROUNDING_2 :[(i32,i32,i32,i32);80] = generate_surroundings_4d();

const CYCLES: u32 = 6;
fn main(){
    let vec = inp::parse_file("day17.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

const fn generate_surroundings_3d() -> [(i32,i32,i32);26] {
    let mut curr_index = 0;
    let mut v = [(0,0,0);26];
    let mut x= -1;
    while x <2 {
        let mut y = -1;
        while y<2{
            let mut z = -1;
            while z<2{
                if !(x==0 && y==0 &&z==0)  {
                    v[curr_index] = (x,y,z);
                    curr_index = curr_index + 1;
                }
                z = z + 1;
            }
            y = y+1;
        }
        x=x+1;
    }
    v
}

const fn generate_surroundings_4d() -> [(i32,i32,i32,i32);80] {
    let mut curr_index = 0;
    let mut v = [(0,0,0,0);80];
    let mut x= -1;
    while x <2 {
        let mut y = -1;
        while y<2{
            let mut z = -1;
            while z<2{
                let mut w = -1;
                while w<2{
                    if !(x==0 && y==0 &&z==0&&w==0)  {
                        v[curr_index] = (x,y,z,w);
                        curr_index = curr_index + 1;
                    }
                    w = w+1;
                }
                z = z + 1;
            }
            y = y+1;
        }
        x=x+1;
    }
    v
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    // Put initial state into board.
    let mut board: HashMap<(i32,i32,i32), bool> = HashMap::new();
    for (y, str) in input.iter().enumerate(){
        let chars = str.chars().collect::<Vec<char>>();
        for x in 0..input[0].len() {
            let mut alive = false;
            if chars[x] == '#' {
                alive = true;
            }
            board.insert((x as i32, y as i32, 0), alive);
        }
    }
    for _cycle in 0..CYCLES {
        //println!("Cycle {}, {} spaces", cycle, board.len());
        // Add surrounding nodes that we might want to check.
        let mut to_add = Vec::new(); // use add list to avoid mutating while iterating.
        for x in board.iter(){
            for change in SURROUNDING.iter() {
                let xcord = x.0;
                let new = (xcord.0 + change.0, xcord.1 + change.1, xcord.2 + change.2);
                if !board.contains_key(&new) {
                    to_add.push((new, false));
                }
            }
        }
        for new_boy in to_add {
            board.insert(new_boy.0, new_boy.1);
        }
        // Change state of nodes.
        to_add = Vec::new();
        for focus in board.iter() {
            let xcord = focus.0;
            let xalive = *focus.1;
            let mut num_alive = 0;
            for change in SURROUNDING.iter() {
                let new = (xcord.0 + change.0, xcord.1 + change.1, xcord.2 + change.2);
                match board.get(&new)  {
                    Some(val) => if *val {
                        num_alive = num_alive+1;
                    },
                    None => (),
                }
            }
            // We should switch to become alive or become dead.
            if (xalive && (num_alive <2 || num_alive >3)) || (!xalive && num_alive == 3){
                to_add.push((*xcord,!xalive));
            }
        }
        for new_boy in to_add {
            board.insert(new_boy.0, new_boy.1);
        }
    }
    let mut num_alive = 0;
    for cube in board {
        if cube.1 {
            num_alive = num_alive + 1;
        }
    }
    println!("Part 1: {}", num_alive);
}

// Solution for part 2
// It's the same as the other one I'm not putting more comments.
fn part2(input: &Vec<String>){
    let mut board: HashMap<(i32,i32,i32,i32), bool> = HashMap::new();
    for (y, str) in input.iter().enumerate(){
        let chars = str.chars().collect::<Vec<char>>();
        for x in 0..input[0].len() {

            let mut alive = false;
            if chars[x] == '#' {
                alive = true;
            }
            board.insert((x as i32, y as i32, 0,0), alive);
        }
    }
    for _cycle in 0..CYCLES {
        //println!("Cycle {}, {} spaces", cycle, board.len());
        let mut to_add = Vec::new();
        for x in board.iter(){
            for change in SURROUNDING_2.iter() {
                let xcord = x.0;
                let new = (xcord.0 + change.0, xcord.1 + change.1, xcord.2 + change.2, xcord.3 + change.3);
                if !board.contains_key(&new) {
                    to_add.push((new, false));
                }
            }
        }
        for new_boy in to_add {
            board.insert(new_boy.0, new_boy.1);
        }
        to_add = Vec::new();
        for focus in board.iter() {
            let xcord = focus.0;
            let xalive = *focus.1;

            let mut num_alive = 0;
            for change in SURROUNDING_2.iter() {
                let new = (xcord.0 + change.0, xcord.1 + change.1, xcord.2 + change.2,xcord.3+change.3);
                match board.get(&new)  {
                    Some(val) => if *val {
                        num_alive = num_alive+1;
                    },
                    None => (),
                }
            }
            if (xalive && (num_alive <2 || num_alive >3)) || (!xalive && num_alive == 3){
                to_add.push((*xcord,!xalive));
            }
        }
        for new_boy in to_add {
            board.insert(new_boy.0, new_boy.1);
        }
    }
    let mut num_alive = 0;
    for cube in board {
        if cube.1 {
            num_alive = num_alive + 1;
        }
    }
    println!("Part 2: {}", num_alive);
}