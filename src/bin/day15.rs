use std::collections::HashMap;

fn main(){
    let vec = inp::parse_file("day15.txt");
    // Put the code to do the thing here
    part1and2(&vec[0]);
}

// Solution for part 1
fn part1and2(input: &String) {
    let inp : Vec<i32>= input.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    println!("Part 1: {}", game(&inp, 2020));
    println!("Part 2: {}", game(&inp, 30000000));
}

fn game(start: &Vec<i32>, target: i32) -> i32{
    println!("Please wait...");
    let mut hash: HashMap<i32, i32> = HashMap::new();
    let mut turn = 1;
    for thing in start {
        hash.insert(*thing, turn);
        turn = turn + 1;
    }
    let mut spoken = 0; // previously spoken number, starts at 0 because we just inserted
    while turn != target {
        match hash.get(&spoken) {
            // Has been spoken before
            Some(x) => {
                let temp = turn-x;
                hash.insert(spoken, turn);
                spoken = temp;
            },
            // Has not been spoken before
            None => {
                hash.insert(spoken, turn);
                spoken = 0;
            },
        }
        turn = turn + 1;
    }
    spoken
}