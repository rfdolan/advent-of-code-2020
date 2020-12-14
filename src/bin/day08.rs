use std::vec::Vec;

const JMP: &str = "jmp";
const ACC: &str = "acc";
const NOP: &str = "nop";

fn main(){
    let vec = inp::parse_file("day8.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let mut visited: Vec<i32> = Vec::new();
    let mut acc: i32 = 0;
    let mut pos: i32 = 0;

    loop {
        let instruction: Vec<String> = input[pos as usize].split(" ").map(|s| s.to_string()).collect();
        let jkl: (&String, i32) = (&instruction[0], instruction[1].parse().expect("Not a number"));
        if visited.contains(&pos) {
            break;
        }
        visited.push(pos);

        let (res_acc, res_pos) = do_op(jkl.0, acc, pos, jkl.1, false);
        acc = res_acc;
        pos = res_pos;
    }
    println!("Part 1: {}", acc);
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let mut flipped:Vec<i32> = Vec::new(); // Vector of positions we tried to flip
    loop {
        let mut try_replace = true; // Should we try to flip an instruction
        let mut visited: Vec<i32> = Vec::new(); // Vector of visited positions
        let mut acc: i32 = 0;
        let mut pos: i32 = 0;
        loop {
            if pos as usize == input.len() {
                println!("Part 2: {}", acc);
                return;
            }
            else if pos as usize > input.len() || pos < 0 {
                break;
            }
            let instruction: Vec<String> = input[pos as usize].split(" ").map(|s| s.to_string()).collect();
            let jkl: (&String, i32) = (&instruction[0], instruction[1].parse().expect("Not a number"));
            
            // If we revisit, we're looping, so break and try again.
            if visited.contains(&pos) {
                break;
            }
            visited.push(pos);

            // If we have not tried to flip this iteration and we have not tried to flip this position, do it
            if try_replace && !flipped.contains(&pos){
                let (res_acc, res_pos) = do_op(jkl.0, acc, pos, jkl.1, true);
                if jkl.0 != ACC {
                    try_replace = false;
                    flipped.push(pos);
                }
                acc = res_acc;
                pos = res_pos;
            }

            // Operate as normal
            else {
                let (res_acc, res_pos) = do_op(jkl.0, acc, pos, jkl.1, false);
                acc = res_acc;
                pos = res_pos;
            }
        }
    }
}

// Does the specified operation, swapping the functionality of JMP and NOP if swap is true
// Returns a tuple of the updated accumulator and position
fn do_op(op: &String, acc: i32, pos: i32, val: i32, swap: bool) -> (i32, i32) {
    if op == ACC {
        return (acc + val, pos + 1);
    }
    else if (op == JMP && !swap) || (op == NOP && swap) {
        return (acc, pos+val);
    }
    else {
        return (acc, pos + 1);
    }

}