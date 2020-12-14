use std::vec::Vec;
mod inp;

const PREAMBLE_LEN: u64 = 25;

fn main(){
    let vec = inp::parse_file("day9.txt").iter().map(|x| x.parse().unwrap()).collect();
    // Put the code to do the thing here
    part1and2(&vec);
}

// Solution for part 1
fn part1and2(input: &Vec<u64>) {
    let mut part1_ans = 0;
    
    // Part 1
    for x in (PREAMBLE_LEN)..(input.len() as u64){
        if !check_num(&(&input[(x-PREAMBLE_LEN) as usize..(x) as usize]).to_vec(), input[(x) as usize]) {
            //println!("Index: {}", x);
            println!("Part 1: {}", input[x as usize]);
            part1_ans = input[x as usize];
            break;
        }
    }

    // Part 2
    for x in 0..input.len() {
        let mut acc = input[x as usize];
        // Add more and more numbers in a line until we either go over or hit it exactly
        for i in x+1..input.len() {
            acc = acc + input[i as usize];
            // If we are too big stop this iteration
            if acc > part1_ans {
                break;
            }
            // We got it
            if acc == part1_ans {
                let min: u64 = *input[x as usize..i].iter().min().unwrap();
                let max: u64 = *input[x as usize..i].iter().max().unwrap();
                println!("Part 2: {}", min+ max);
                return;
            }
        }
    }
}

fn check_num(vec: &Vec<u64>, num: u64 ) -> bool {
    for x in 0..PREAMBLE_LEN{
        for y in 0..PREAMBLE_LEN{
            if x != y {
                //println!("{},{}", vec[x as usize], vec[y as usize]);
                if vec[x as usize]+vec[y as usize] == num {
                    return true;
                }
            }
        }
    }
    return false;
}