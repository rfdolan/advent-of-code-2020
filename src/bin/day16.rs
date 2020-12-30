use std::vec::Vec;
use std::collections::HashMap;
use std::ops::Range;

const NUM_FIELDS: usize = 20; // If you're running on a different sized input or something change this

fn main(){
    let vec = inp::parse_file("day16.txt");
    // Put the code to do the thing here
    part1and2(&vec);
}

fn parse_tickets(input: &Vec<String>) -> (HashMap<String, Vec<Range<i32>>>, Vec<i32>, Vec<Vec<i32>>) {
    let mut fields: HashMap<String, Vec<Range<i32>>> = HashMap::new();
    let mut my_ticket: Vec<i32> = Vec::new();
    let mut all_tickets: Vec<Vec<i32>> = Vec::new();
    let mut cursor = 0;
    
    for x in input.iter() {
        if x == "" {
            cursor = cursor + 2;
            break;
        }
        let split = x.split(": ").collect::<Vec<&str>>();
        let fieldname = split[0];
        let nums = split[1].split(" or ").collect::<Vec<&str>>();
        let mut ranges = Vec::new();
        for y in nums {
            let s = y.split("-").collect::<Vec<&str>>();
            let lower = s[0].parse::<i32>().unwrap();
            let upper = s[1].parse::<i32>().unwrap() + 1; // add 1 to this if it is inclusive
            ranges.push(lower..upper);
        }
        fields.insert(String::from(fieldname), ranges);
        cursor = cursor + 1;
    }
    for x in input[cursor..].iter() {
        if x == "" {
            cursor = cursor + 2;
            break;
        }
        let new_ticket = x.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        all_tickets.push(new_ticket.clone());
        my_ticket = new_ticket;
        cursor = cursor + 1;
    }
    for x in input[cursor..].iter() {
        let new_ticket = x.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        all_tickets.push(new_ticket);
    }
    (fields, my_ticket, all_tickets)
}

/*
fn valid_ticket(fields: &HashMap<String, Vec<Range<i32>>>, ticket: &Vec<i32>) -> bool {
    for val in ticket {
        if !valid_num(*val, &fields) {
            return false;
        }
    }
    return true;
}
*/

//#########################################################################
//# __          __     _____  _   _ _____ _   _  _____ _                  #
//# \ \        / /\   |  __ \| \ | |_   _| \ | |/ ____| |                 #
//#  \ \  /\  / /  \  | |__) |  \| | | | |  \| | |  __| |                 #
//#   \ \/  \/ / /\ \ |  _  /| . ` | | | | . ` | | |_ | |                 #
//#    \  /\  / ____ \| | \ \| |\  |_| |_| |\  | |__| |_|                 #
//#  __ \/_ \/_/__ _\_\_|  \_\_| \_|_____|_| \_|\_____(_)_  __            #
//# |  \/  |  ____/ ____|   /\          | |  /\   | \ | | |/ /            #
//# | \  / | |__ | |  __   /  \         | | /  \  |  \| | ' /             #
//# | |\/| |  __|| | |_ | / /\ \    _   | |/ /\ \ | . ` |  <              #
//# | |  | | |___| |__| |/ ____ \  | |__| / ____ \| |\  | . \             #
//# |_|__|_|______\_____/_/____\_\  \____/_/    \_\_|_\_|_|\_\    _____   #
//#  / ____/ __ \|  __ \|  ____|     /\   | |  | |  ____|   /\   |  __ \  #
//# | |   | |  | | |  | | |__       /  \  | |__| | |__     /  \  | |  | | #
//# | |   | |  | | |  | |  __|     / /\ \ |  __  |  __|   / /\ \ | |  | | #
//# | |___| |__| | |__| | |____   / ____ \| |  | | |____ / ____ \| |__| | #
//#  \_____\____/|_____/|______| /_/    \_\_|  |_|______/_/    \_\_____/  #
//#########################################################################
// Seriously I feel like I'm commiting a crime with this soltion. Don't know why but it's just eugh
// Solution for part 1
fn part1and2(input: &Vec<String>) {
    let (fields, my_ticket, all_tickets) = parse_tickets(input);
    let mut invalid_nums = Vec::new();
    let mut all_valid_tickets = Vec::new();
    for ticket in all_tickets.iter(){
        let mut good = true;
        for val in ticket {
            if !valid_num(*val, &fields) {
                invalid_nums.push(val);
                good = false;
            }
        }
        if good {
            all_valid_tickets.push(ticket.clone());
        }
    }
    println!("Part 1: {}", invalid_nums.iter().fold(0, |acc, x| acc+*x));

    let mut valid_fields: HashMap<String, [bool; NUM_FIELDS]> = HashMap::new();
    let array_of_20: [bool; NUM_FIELDS] = [true; NUM_FIELDS];
    for x in fields.iter() {
        valid_fields.insert(x.0.clone(), array_of_20.clone());
    }
    let keys = fields.keys();
    for ticket in all_valid_tickets.iter() {
        for (i,slot) in ticket.iter().enumerate() {
            for key in keys.clone() {
                if !valid_num_for_field(*slot, fields.get(key).unwrap().clone()) {
                    //println!("{} cant be in position {}", key, i);
                    let mut curr = valid_fields.get(key).unwrap().clone();
                    curr[i] = false;
                    valid_fields.insert(key.clone(), curr);
                }
            }
        }
    }
    let mut really_for_real_final_ordering = Vec::new();
    for _ in 0..NUM_FIELDS {
        really_for_real_final_ordering.push(String::from(""));
    }
    // The number of passes it takes to validate this is the number of items as long as it's solvable. Less maybe even
    for _ in 0..NUM_FIELDS {
        // It's this part right here that I feel bad about
        let new_valid = valid_fields.clone();
        let mut curr_to_remove = 0;
        for field in new_valid.iter() {
            if get_num_true(*field.1) == 1 {
                for (i,val) in new_valid.get(field.0).unwrap().iter().enumerate() {
                    if *val {
                        let buh = field.0.clone();
                        really_for_real_final_ordering[i] =  buh;
                        curr_to_remove = i;
                        break;
                    }
                }
                break;
            }
        }
        for field in new_valid.iter() {
            let mut old = valid_fields.get(field.0).unwrap().clone();
            old[curr_to_remove] = false;
            valid_fields.insert(field.0.clone(), old);
        }
    }
    let mut acc:i64 = 1;
    for (i,x) in really_for_real_final_ordering.iter().enumerate() {
        //println!("{}: {}: {}",i,x,my_ticket[i]);
        if x.contains("departure") {
            acc = acc * my_ticket[i] as i64;
        }
    }
    
    println!("Part 2: {}", acc);
}

fn valid_num(num: i32, fields: &HashMap<String, Vec<Range<i32>>>) -> bool {
    for field in fields.iter() {
        for range in field.1 {
            if range.contains(&num) {
                return true;
            }
        }
    }
    return false
}

fn valid_num_for_field(num: i32, field: Vec<Range<i32>>) -> bool {
    for range in field {
        if range.contains(&num) {
            return true;
        }
    }
    return false;
}

fn get_num_true(arr: [bool;NUM_FIELDS]) -> i32 {
    let mut acc = 0;
    for x in arr.iter() {
        if *x {
            acc = acc + 1;
        }
    }
    acc
}
