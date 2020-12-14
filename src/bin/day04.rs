use std::vec::Vec;
use std::ops::Range;
use std::collections::HashMap;
mod inp;

const BYR_RANGE:Range<u32> = 1920..2003;
const IYR_RANGE:Range<u32> = 2010..2021;
const EYR_RANGE:Range<u32> = 2020..2031;
const HGT_RANGE_CM:Range<u32> = 150..194;
const HGT_RANGE_IN:Range<u32> = 59..77;
const HCL_VALUES:[char; 17] = ['#','a','b','c','d','e','f','0','1','2','3','4','5','6','7','8','9'];

const ECL_VALUES: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const PID_SIZE: u32 = 9;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];



fn main(){
    let vec = inp::parse_file("day4.txt");
    // Put the code to do the thing here
    let mut passports = oneline_input(&vec);
    part1(&mut passports);
    part2(&mut passports);
}

// Solution for part 1
fn part1(passports: &mut Vec<String>) {
    passports.retain(|s|has_all_req(s));

    println!("Part 1: {}", passports.len());
}

fn oneline_input(vec: &[String]) -> Vec<String> {
    let mut curr: String = "".to_string();
    let mut res = Vec::new();
    for line in vec {
        if line == "" {
            res.push(curr.trim().to_string());
            curr = "".to_string();
        }
        else {
            curr = curr + line + " ";
        }
    }
    res.push(curr.trim().to_string());
    return res;
}

fn has_all_req(passport: &String) -> bool {
    // I'm sorry
    if passport.contains("byr:") && passport.contains("iyr:") && passport.contains("eyr:") 
    && passport.contains("hgt:") && passport.contains("hcl:") && passport.contains("ecl:") 
    && passport.contains("pid:") {
        return true;
    }
    return false;
}

// Solution for part 2
fn part2(passports: &mut Vec<String>) {


    passports.retain(|s|has_all_req(s));
    let mut valid_passports =0;
    for passport in passports {
        // uhh split up into sublist of fields
        let split = passport.split(" ");
        // map fields to  values in map??
        let mut data = HashMap::new();
        for item in split {
            let mut item_split = item.split(":");
            let key = item_split.nth(0).unwrap();
            //println!("[{}]", item);// Solution for part 2
            let val = item_split.nth(0).unwrap();
            data.insert(
                key,
                val,
            );
        }
        if !is_valid(data) {
        } else {
            //println!("{}", passport);

            valid_passports  = valid_passports + 1;
        }
    }
    println!("Part 2: {}", valid_passports);
}

fn is_valid(data: HashMap<&str, &str>) -> bool {
    // check?? profit??
    for field in FIELDS.iter() {
        if field == &"byr"{
            match data.get(field) {
                Some(result) => {
                    
                    //println!("Birth year {} invalid", result);
                    if !BYR_RANGE.contains(&result.parse().expect("Not a number")){
                        //println!("INVALID^^^");
                        return false;
                    }
                },
                None => return false,
            }
        }
        if field == &"iyr"{
            match data.get(field) {
                Some(result) => if !IYR_RANGE.contains(&result.parse().expect("Not a number")){
                    //println!("Issue year {} invalid", result);
                    return false;
                },
                None => return false,
            }
        }
        if field == &"eyr"{
            match data.get(field) {
                Some(result) => if !EYR_RANGE.contains(&result.parse().expect("Not a number")){
                    //println!("Expiry year {} invalid", result);
                    return false;
                },
                None => return false,
            }
        }
        if field == &"hgt"{
            match data.get(field) {
                Some(result) => {
                    //println!("{}", result);
                    if result.chars().rev().take(2).collect::<String>() == *"ni" {
                        let mut s = result.to_string();
                        let len = s.len();
                        s.truncate(len-2);
                        //println!("{}in", s);
                        if !HGT_RANGE_IN.contains(&s.parse().expect("Not a number")){
                            //println!("INVALID^^");
                            //println!("Height {} invalid", result);
                            return false;
                        }
                    }
                    else if result.chars().rev().take(2).collect::<String>() == *"mc"{
                        let mut s = result.to_string();
                        let len = s.len();
                        s.truncate(len-2);
                        //println!("{}cm", s);
                        
                        if !HGT_RANGE_CM.contains(&s.parse().expect("Not a number")){
                            //println!("INVALID^^");
                            //println!("Height {} invalid", result);
                            return false;
                        }
                    } else { return false;}
                },
                None => return false,
            }
        }
        if field == &"hcl"{
            let mut first = true;
            match data.get(field) {
                Some(result) => {
                    for c in result.chars() {
                        if first {
                            first = false;
                            if c != '#' {
                                //println!("HCL {} doesn't start with #", result);
                                return false;
                            }
                        }
                        if !HCL_VALUES.contains(&c){
                            //println!("Hair color {} invalid", result);
                            return false;
                        }
                    } if result.len() != 7 { return false }
                    },

                None => return false,
            }
        }
        if field == &"ecl"{
            match data.get(field) {
                Some(result) => if !ECL_VALUES.contains(&result){
                    //println!("Eye color {} invalid", result);
                    return false;
                },
                None => return false,
            }
        }
        if field == &"pid"{
            match data.get(field) {
                Some(result) => if PID_SIZE != result.len() as u32{
                    //println!("PID {} invalid", result);
                    return false;
                },
                None => return false,
            }
        }
    }

    return true;
}