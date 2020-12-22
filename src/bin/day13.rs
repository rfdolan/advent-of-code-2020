use std::vec::Vec;

fn main(){
    let vec = inp::parse_file("day13.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let earliest_time: u32 = input[0].parse().unwrap();
    let mut curr_time = earliest_time;
    let split = input[1].split(",").map(|s| s.to_string()).collect::<Vec<String>>();
    let jkl = split.iter().filter(|&s| s!="x");
    let jkl1 : Vec<u32> = jkl.map(|s| s.parse().unwrap()).collect();

    loop {
        for bus_num in &jkl1[..] {
            if curr_time % bus_num == 0 {
                println!("Part 1: {}", (curr_time - earliest_time) * bus_num);
                return;
            }
        }
        curr_time = curr_time + 1;
    }
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let split = input[1].split(",").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut busses = Vec::new();
    for (i,x) in split.iter().enumerate() {
        if x != "x" {
            busses.push((i,x.parse().unwrap()));
        }
    }
    // Based Chinese Remainder Theorem
    let mut n = 1;
    for x in busses.iter() {
        n=n*x.1;
    }
    let mut total = 0;
    for bus in busses.iter() {
        total = total + calculate_crt(*bus,n);
    }

    println!("Part 2: {}", ((total%n)+n)%n); // I hate you rust "modulo" operator
}

fn calculate_crt(bus: (usize, i128), n:i128) -> i128 {
    let b = -1 * bus.0 as i128;
    let ni = n/bus.1;
    // Ni*x = 1 (mod bus.1)
    let times_x = ni % bus.1;
    let mut curr = times_x;
    let mut times = 1;
    loop {
        if curr % bus.1 == 1{
            return b*ni*times;
        }
        curr = curr + times_x;
        times = times + 1;
    }
}