use std::vec::Vec;
use std::collections::VecDeque;
use std::fmt;

enum Victory {
    P1Win,
    P2Win,
}

impl fmt::Display for Victory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Victory::P1Win => write!(f, "CRAB wins!"),
           Victory::P2Win => write!(f, "YOU win!"),
       }
    }
}

fn parse_decks(input: &Vec<String>) -> (VecDeque<i32>, VecDeque<i32>) {
    let mut p1_deck = VecDeque::new();
    let mut p2_deck = VecDeque::new();
    let mut focus_on_p1 = true;
    for x in input{
        if x == "Player 2:" {
            focus_on_p1 = false;
        } else {
            match x.parse::<i32>() {
                Ok(num) => {
                    if focus_on_p1 {
                        p1_deck.push_back(num);
                    } else {
                        p2_deck.push_back(num);
                    }
                },
                Err(_e) => ()
            }
        }
    }
    (p1_deck, p2_deck)
}

fn do_turn_cc(deck1: VecDeque<i32>,deck2: VecDeque<i32>) -> (VecDeque<i32>, VecDeque<i32>) {
    let (mut p1_deck, mut p2_deck) = (deck1.clone(), deck2.clone());
    let p1_card = p1_deck.pop_front().unwrap();
    let p2_card = p2_deck.pop_front().unwrap();
    if p1_card > p2_card {
        p1_deck.push_back(p1_card);
        p1_deck.push_back(p2_card);
        return (p1_deck.clone(), p2_deck.clone());
    }
    p2_deck.push_back(p2_card);
    p2_deck.push_back(p1_card);
    return (p1_deck.clone(), p2_deck.clone());
}

fn crab_combat(deck1: VecDeque<i32>, deck2: VecDeque<i32>) -> (Victory, VecDeque<i32>) {
    let (mut p1_deck, mut p2_deck) = (deck1.clone(), deck2.clone());
    loop{
        let (res1, res2) = do_turn_cc(p1_deck, p2_deck);
        p1_deck = res1;
        p2_deck = res2;
        if p1_deck.len() == 0 {
            return (Victory::P2Win, p2_deck);
        } 
        if p2_deck.len() == 0 {
            return (Victory::P1Win, p1_deck);
        }
    }
}

// Solution for part 1
fn part1(input: &Vec<String>) {
    let (p1_deck, p2_deck) = parse_decks(input);
    let (winner, mut deck) = crab_combat(p1_deck, p2_deck);
    let mut score = 0;
    let mut index = 1;    
    while deck.len() > 0 {
        score = score + (deck.pop_back().unwrap() * index);
        index = index + 1;
    }
    println!("{}", winner);
    println!("Part 1: {}", score);
}

fn do_turn_rc(deck1: VecDeque<i32>,deck2: VecDeque<i32>) -> Victory {
    let (mut p1_deck, mut p2_deck) = (deck1.clone(), deck2.clone());
    let p1_card = p1_deck.pop_front().unwrap();
    let p2_card = p2_deck.pop_front().unwrap();
    // If both players have at least as many cards remaining in their deck as the value of the card they just drew, 
    //   the winner of the round is determined by playing a new game of Recursive Combat
    if p1_deck.len() >= p1_card as usize && p2_deck.len() >= p2_card as usize {
        let mut next1 = VecDeque::new();
        let mut next2 = VecDeque::new();
        for _ in 0..p1_card as usize {
            next1.push_back(p1_deck.pop_front().unwrap());
        }
        for _ in 0..p2_card as usize {
            next2.push_back(p2_deck.pop_front().unwrap());
        }
        let (winner, _deck) = recursive_combat(next1,next2);
        return winner;
    }
    // Otherwise, at least one player must not have enough cards left in their deck to recurse; the winner of the round is 
    //   the player with the higher-value card.
    else if p1_card > p2_card {
        return Victory::P1Win;
    }
    return Victory::P2Win;
}

fn recursive_combat(deck1: VecDeque<i32>, deck2: VecDeque<i32>) -> (Victory,  VecDeque<i32>) {
    let (mut d1, mut d2) = (deck1.clone(), deck2.clone());
    let mut previous_decks = Vec::new();
    loop{
        // Chech for identical decks within this game
        if previous_decks.contains(&(d1.clone(), d2.clone())) {
            return (Victory::P1Win, d1);
        }
        previous_decks.push((d1.clone(), d2.clone()));
        let res = do_turn_rc(d1.clone(), d2.clone());
        let card1 = d1.pop_front().unwrap();
        let card2 = d2.pop_front().unwrap();
        match res {
            Victory::P1Win => {
                d1.push_back(card1);
                d1.push_back(card2);
            },
            Victory::P2Win => {
                d2.push_back(card2);
                d2.push_back(card1);
            }
        }
        if d1.len() == 0 {
            return (Victory::P2Win, d2);
        } 
        if d2.len() == 0 {
            return (Victory::P1Win, d1);
        }
    }
}

// Solution for part 2
fn part2(input: &Vec<String>) {
    let (p1_deck, p2_deck) = parse_decks(input);
    let (winner, mut res) = recursive_combat(p1_deck,p2_deck);
    let mut score = 0;
    let mut index = 1;    
    while res.len() > 0 {
        score = score + (res.pop_back().unwrap() * index);
        index = index + 1;
    }
    println!("{}",winner);
    println!("Part 2: {}", score);
}

fn main(){
    let vec = inp::parse_file("day22.txt");
    // Put the code to do the thing here
    part1(&vec);
    part2(&vec);
}