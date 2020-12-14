use std::vec::Vec;
mod inp;

const ROW_SIZE: u32 = 127;
const COL_SIZE:u32 = 7;

fn main(){
    let vec = inp::parse_file("day5.txt");
    // Put the code to do the thing here
    part1and2(&vec);
}

// Solution for part 1
fn part1and2(tickets: &Vec<String>) {
    let mut seating_arr = Vec::new();
    for ticket in tickets {
        let mut rowmax = ROW_SIZE;
        let mut rowmin =0;
        let mut colmax = COL_SIZE;
        let mut colmin = 0;
        // Do the thing
        for c in ticket.chars()  {
            let rowdif = rowmax-rowmin;
            let coldif = colmax-colmin;
            match c {
                'B' => rowmin = rowmin + (rowdif/2)+1,
                'F' => rowmax = rowmin + (rowdif/2),
                'R' => colmin = colmin + (coldif/2)+1,
                'L' => colmax = colmin + (coldif/2),
                _ => println!("what"),
            }

        }
        seating_arr.push((rowmin, colmin));
        //println!("({}, {})", rowmin, colmin);
    }
    let mut biggest = 0;
    let mut seat_ids = Vec::new();

    // Find the biggest seat id
    for seat in seating_arr {
        let seatid = get_seat_id(seat);
        seat_ids.push(seatid);
        if seatid > biggest {
            biggest = seatid;
        }
    }
    println!("Part 1: {}", biggest);

    // Sort the list of ids
    seat_ids.sort();
    let mut res = 0;
    let mut prev = 0;

    // Go through and find the missing seat
    for id in seat_ids{
        if id - prev != 1 && prev != 0 {
            res = id-1;
        }
        prev = id;
    }
    println!("Part 2: {}", res);

}

// Get the seat id from the given row/column
fn get_seat_id(pos: (u32, u32)) -> u32 {
    let (row,col) = pos;
    (row * 8) + col
}