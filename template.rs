use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main(){
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines("day-x.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let line_num: u32 = ip.parse().expect("Not a number!");
                vec.push(line_num);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}