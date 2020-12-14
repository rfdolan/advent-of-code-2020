use std::vec::Vec;


fn main() {
    let vec = inp::parse_file("day1.txt").iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(vec: &Vec<i32>) {
    for int1 in vec.iter() {
        let target = 2020 - int1;
        if vec.contains(&target) {
            println!("Part 1: {}", int1*target);
            return;
        }
    }
}

// Solution for part 2
fn part2(vec: &Vec<i32>) {
    for int1 in vec.iter() {
        for int2 in vec.iter() {
            let target = 2020 - int1 -int2;
            if vec.contains(&target) {
                println!("Part 2: {}", int1*int2*target);
                return;
            }
        }
    }
}