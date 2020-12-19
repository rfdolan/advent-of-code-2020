use std::vec::Vec;

#[derive(Copy, Clone)]
struct Cube {
    coordinates: (i32, i32, i32),
    alive: bool,
}
#[derive(Copy, Clone)]
struct Cube_2 {
    coordinates: (i32, i32, i32,i32),
    alive: bool,
}

static mut SURROUNDING :Vec<(i32,i32,i32)> = Vec::new();
static mut SURROUNDING_2 :Vec<(i32,i32,i32,i32)> = Vec::new();

const CYCLES: u32 = 6;
fn main(){
    let vec = inp::parse_file("day17.txt");
    unsafe {
        generate_surrounding();
    }
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

unsafe fn generate_surrounding() {
    let mut v = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if !(x==0 && y==0 &&z==0) & !v.contains(&(x,y,z)) {
                    v.push((x,y,z));
                }
            }
        }
    }
    SURROUNDING = v;
    let mut v2 = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in -1..2 {
                    if !(x==0 && y==0 &&z==0 && w==0) & !v2.contains(&(x,y,z,w)) {
                        v2.push((x,y,z,w));
                    }
                }
            }
        }
    }
    SURROUNDING_2 = v2;
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let mut board: Vec<Cube> = Vec::new();
    for (y, str) in input.iter().enumerate(){
        let chars = str.chars().collect::<Vec<char>>();
        for x in 0..input[0].len() {

            let mut alive = false;
            if chars[x] == '#' {
                alive = true;
                //println!("Cube {},{},0 is alive",x,y);
            }
            let cube = Cube {coordinates: (x as i32,y as i32,0), alive: alive };
            board.push(cube);
        }
    }
    for cycle in 0..CYCLES {
        println!("Cycle {}, {} spaces", cycle, board.len());
        let mut old = deepcopy(&board);
        let older = deepcopy(&board);
        for x in older {
            unsafe{
                for change in SURROUNDING.iter() {
                    let xcord = x.coordinates;
                    let new = (xcord.0 + change.0, xcord.1 + change.1, xcord.2 + change.2);
                    if !contains_node(&old, new) {
                        old.push(Cube {coordinates: new, alive: false});
                    }
                }
            }
        }
        board = Vec::new();
        for focus in old.iter() {
            let old_again = deepcopy(&old);
            if should_switch(&focus, old_again){
                let new_cube = Cube{coordinates: (focus.coordinates.0, focus.coordinates.1, focus.coordinates.2), alive: !focus.alive};
                //println!("Cube {},{},{}, becomes {}", focus.coordinates.0,focus.coordinates.1, focus.coordinates.2, focus.alive);
                board.push(new_cube);
            } else {
                let new_cube = Cube{coordinates: (focus.coordinates.0, focus.coordinates.1, focus.coordinates.2), alive: focus.alive};
                board.push(new_cube);
            }
        }
    }
    let mut num_alive = 0;
    for cube in board {
        if cube.alive {
            //println!("Cube at {},{},{} is {}", cube.coordinates.0, cube.coordinates.1, cube.coordinates.2, cube.alive);
            num_alive = num_alive + 1;
        }
    }

    println!("Part 1: {}", num_alive);
}

fn contains_node(board: &Vec<Cube>, cube: (i32,i32,i32)) -> bool {
    for x in board {
        let cord = x.coordinates;
        if cord == cube {
            return true
        }
    }
    return false;

}

// I'm sorry god
fn deepcopy(original: &Vec<Cube>) -> Vec<Cube> {
    let mut new_vec = Vec::new();
    for cube in original {
        new_vec.push(Cube {coordinates: (cube.coordinates.0, cube.coordinates.1, cube.coordinates.2), alive: cube.alive});
    }
    new_vec
}

fn should_switch(focus: &Cube, board: Vec<Cube>) -> bool {
    let mut num_alive = 0;
    //println!("{}", board.len());
    for cube in board {
        if are_neighbors(&focus, &cube) && cube.alive{
            num_alive = num_alive + 1;
        }
    }
    if focus.alive && (num_alive <2 || num_alive >3){
        return true;
    }
    else if !focus.alive && num_alive == 3 {
        return true;
    }

    return false;
}

fn are_neighbors(c1: &Cube, c2: &Cube) -> bool {
    let cord1 = c1.coordinates;
    let cord2 = c2.coordinates;
    if (cord1.0-cord2.0).abs() < 2 && (cord1.1 - cord2.1).abs() < 2 && (cord1.2-cord2.2).abs() < 2 {
        if !(cord1.0==cord2.0 && cord1.1==cord2.1 && cord1.2 == cord2.2) {
            return true;
        }
    }
    return false;
}

// Solution for part 2
fn part2(input: &Vec<String>){
    let mut board: Vec<Cube_2> = Vec::new();
    for (y, str) in input.iter().enumerate(){
        let chars = str.chars().collect::<Vec<char>>();
        for x in 0..input[0].len() {
            let mut alive = false;
            if chars[x] == '#' {
                alive = true;
            }
            let cube = Cube_2 {coordinates: (x as i32,y as i32,0,0), alive: alive };
            board.push(cube);
        }
    }
    for cycle in 0..CYCLES {
        println!("Cycle {}, {} spaces", cycle, board.len());
        let mut old = deepcopy_2(&board);
        for x in board.iter() {
            unsafe{
                for change in SURROUNDING_2.iter() {
                    let xcord = x.coordinates;
                    let new = (xcord.0 + change.0, xcord.1 + change.1, xcord.2 + change.2, xcord.3+change.3);
                    if !contains_node_2(&old, new) {
                        old.push(Cube_2 {coordinates: new, alive: false});
                    }
                }
            }
        }
        board = Vec::new();
        // monster loop that makes this take so long, only n^2 I think but hoo boy
        for focus in old.iter() {
            if should_switch_2(&focus, &old){
                let new_cube = Cube_2{coordinates: (focus.coordinates.0, focus.coordinates.1, focus.coordinates.2, focus.coordinates.3), alive: !focus.alive};
                //println!("Cube {},{},{}, becomes {}", focus.coordinates.0,focus.coordinates.1, focus.coordinates.2, focus.alive);
                board.push(new_cube);
            } else {
                let new_cube = Cube_2{coordinates: (focus.coordinates.0, focus.coordinates.1, focus.coordinates.2, focus.coordinates.3), alive: focus.alive};
                board.push(new_cube);
            }
        }
    }
    let mut num_alive = 0;
    for cube in board {
        if cube.alive {
            //println!("Cube at {},{},{} is {}", cube.coordinates.0, cube.coordinates.1, cube.coordinates.2, cube.alive);
            num_alive = num_alive + 1;
        }
    }
    println!("Part 2: {}", num_alive);
}

// I'm sorry god again
fn deepcopy_2(original: &Vec<Cube_2>) -> Vec<Cube_2> {
    let mut new_vec = Vec::new();
    for cube in original {
        new_vec.push(Cube_2 {coordinates: (cube.coordinates.0, cube.coordinates.1, cube.coordinates.2, cube.coordinates.3), alive: cube.alive});
    }
    new_vec
}

fn contains_node_2(board: &Vec<Cube_2>, cube: (i32,i32,i32,i32)) -> bool {
    for x in board {
        let cord = x.coordinates;
        if cord == cube {
            return true
        }
    }
    return false;
}

fn should_switch_2(focus: &Cube_2, board: &Vec<Cube_2>) -> bool {
    let mut num_alive = 0;
    //println!("{}", board.len());
    for cube in board {
        if are_neighbors_2(&focus, &cube) && cube.alive{
            num_alive = num_alive + 1;
        }
    }
    if focus.alive && (num_alive <2 || num_alive >3){
        return true;
    }
    else if !focus.alive && num_alive == 3 {
        return true;
    }

    return false;
}

fn are_neighbors_2(c1: &Cube_2, c2: &Cube_2) -> bool {
    let cord1 = c1.coordinates;
    let cord2 = c2.coordinates;
    if cord1.0==cord2.0 && cord1.1==cord2.1 && cord1.2==cord2.2 && cord1.3 == cord2.3 {
        return false;
    }
    if (cord1.0-cord2.0).abs() <2 && (cord1.1-cord2.1).abs() < 2 && (cord1.2 - cord2.2).abs() < 2 && (cord1.3-cord2.3).abs() < 2 {
        return true;
    }
    return false;
}