use std::vec::Vec;

fn main(){
    let vec = inp::parse_file("day6.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(answers: &Vec<String>) {
    let mut total_group = String::from("");
    let mut total_groups= Vec::new();
    for answer in answers{
        if answer == "" {
            let buh = total_group.clone();
            total_groups.push(buh);
            total_group = String::from("");
        }
        else {
            total_group = total_group + answer;
        }
    }
    total_groups.push(total_group);
    let mut acc = 0;
    let mut different_questions = String::from("");
    for group in total_groups {
        for c in group.chars() {
            if !different_questions.contains(c) && c != ' ' {
                different_questions.push(c);
            }
        }
        acc = acc + different_questions.len();
        different_questions = String::from("");
    }

    println!("Part 1: {}", acc);
}

// Solution for part 2
fn part2(answers: &Vec<String>) {
    let mut answered_questions = String::from("");
    let mut prev = "";
    let mut acc = 0;
    for person in answers {
        //println!("{}",person);
        if answered_questions == ""  && prev == ""{
            answered_questions = person.to_string();
        }
        if person == "" {
            acc = acc + answered_questions.len();
            answered_questions = String::from("");
        }
        else {
            let mut new_answered = answered_questions.clone();
            for c in answered_questions.chars() {
                if !person.contains(c) {
                    new_answered = str::replace(&new_answered, c, "");
                }
            }
            answered_questions = new_answered;

        }
        prev = person;

    }
    acc = acc + answered_questions.len();

    println!("Part 2: {}", acc);
}