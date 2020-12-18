use std::vec::Vec;

// I don't like trees so I decided to use the Shunting Yard Algorithm (seen in the two rev_polish fns). 
// It's pretty dope

// Given equation in reverse polish, evaluate
fn do_equation(eq: &String) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    let mut input = eq.chars().rev().collect::<Vec<char>>();
    while input.len() > 0 {
        let c = input.pop().unwrap();
        if c.is_digit(10) {
            stack.push(c.to_digit(10).unwrap() as i64);
        } else {
            let op1 = stack.pop().unwrap();
            let op2 = stack.pop().unwrap();
            if c == '+' {
                stack.push(op1+op2);
            } else if c == '*' {
                stack.push(op1*op2);
            }
        }
    }
    return stack.pop().unwrap();
}

// trust me
// This basically reverses the equation. I was having trouble with evaluating and this fixed it. idk man
fn create_monster(original: &String) -> String {
    let mut true_eq = original.chars().rev().collect::<String>();
    true_eq = true_eq.replace("(", "b");
    true_eq = true_eq.replace(")", "(");
    true_eq = true_eq.replace("b", ")");
    true_eq
}

// Parse equation into reverse polish notation
fn rev_polish(eq: &String) -> String {
    let true_eq = create_monster(eq);
    let mut op_stack = Vec::new();
    let mut output_queue = Vec::new();
    for c in true_eq.chars(){
        if c.is_digit(10) { // push digit directly to output stack
            output_queue.push(c);
        }else if c == '('{ // push left paren
            op_stack.push(c);
        } else if c == ')' { // pop until we find left paren
            while op_stack[op_stack.len() - 1] != '(' {
                let op = op_stack.pop().unwrap();
                output_queue.push(op);
            }
            op_stack.pop(); // get rid of paren
        } else { // no precedence, no problems
            op_stack.push(c);
        }
    }
    while op_stack.len() > 0 {
        let op = op_stack.pop().unwrap();
        output_queue.push(op);
    }
    output_queue.into_iter().collect::<String>()
}

// Parse equation into reverse polish notation, taking into account precedence rules
fn rev_polish_2(eq: &String) -> String {
    let true_eq = create_monster(eq);
    let mut op_stack = Vec::new(); // stack for operators
    let mut output_queue = Vec::new(); // queue for final output
    for c in true_eq.chars(){
        if c.is_digit(10) { // If it's a digit sauce it in the output queue
            output_queue.push(c);
        }else if c=='*' {// + has higher precedence, so push all those over before we add *
            while op_stack.len() > 0 && op_stack[op_stack.len() -1] == '+' {
                let op = op_stack.pop().unwrap();
                output_queue.push(op);
            }
            op_stack.push(c);
        }else if c=='+' {// just push, he's the boss
            op_stack.push(c);
        }
        else if c == '('{ // if it is a left paren, push it
            op_stack.push(c);
        } else if c == ')' { // if it is a right paren, pop onto output until we hit the corresponding paren
            while op_stack[op_stack.len() - 1] != '(' {
                let op = op_stack.pop().unwrap();
                output_queue.push(op);
            }
            op_stack.pop(); // get rid of paren
        }
    }
    // Pop the remaining operators
    while op_stack.len() > 0 {
        let op = op_stack.pop().unwrap();
        output_queue.push(op);
    }
    output_queue.into_iter().collect::<String>()
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let mut total = 0;
    for eq in input {
        total = total + do_equation(&rev_polish(&eq));

    }
    println!("Part 1: {}", total);
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let mut total = 0;
    for eq in input {
        total = total + do_equation(&rev_polish_2(&eq));
    }
    println!("Part 2: {}", total);
}

fn main(){
    let vec = inp::parse_file("day18.txt").iter().map(|x| x.replace(" ", "")).collect::<Vec<String>>();
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}