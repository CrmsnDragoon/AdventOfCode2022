#![feature(exclusive_range_pattern)]

use std::collections::VecDeque;

#[derive(Debug,Copy, Clone)]
enum Crate{
    Empty,
    Filled(char)
}

fn main() {
    println!(
        "answer one: {}",
        answer_one(include_str!("../input/input.txt"))
    );
    println!(
        "answer two: {}",
        answer_two(include_str!("../input/input.txt"))
    );
}

#[derive(Debug, Copy, Clone)]
struct Move {
    count: usize,
    origin: usize,
    destination: usize,
}

fn get_moves(input: &str) -> Vec<Move> {
    input.trim().lines().map(|line| {
        let mut count: Option::<usize> = None;
        let mut origin: Option::<usize> = None;
        let mut destination: Option::<usize> = None;
        line.split(' ').for_each(|seg| {
            if seg.chars().next().unwrap().is_numeric() {
                if seg.parse::<i32>().is_ok() {
                    if count.is_none() {
                        count = Some(seg.parse().unwrap());
                    } else if origin.is_none() {
                        origin = Some(seg.parse().unwrap());
                    } else if destination.is_none() {
                        destination = Some(seg.parse().unwrap());
                    }
                }
            }
        });
        Move {
            count: count.unwrap(),
            origin: origin.unwrap(),
            destination: destination.unwrap(),
        }
    }).collect()
}

fn get_crates(input: &str) -> Vec<VecDeque<Crate>> {
    let mut longest_line = 0;
    input.lines().for_each(|line| {
        if line.len() > longest_line {
            longest_line = line.len();
        }
    });
    let mut columns = 0;
    let mut rows = 0;
    input.lines().enumerate().for_each(|(line_num, mut line)|{
        line = line.trim();
        let initial_char = line.chars().next().unwrap();
        match initial_char{
            '['=>{}
            _=>{
                columns = line.split("  ").last().unwrap().trim().parse().unwrap();
                rows = line_num;
            }
        }
    });
    println!("columns: {}", columns);
    println!("initial rows: {}", rows);
    let mut stacks = vec!();
    for x in 0..columns{
        stacks.push(VecDeque::<Crate>::new())
    };
    let mut stack = input.lines().rev();
    stack.next();
    stack.for_each(|line| {
        line.chars().enumerate().for_each(|(index, character)| {
            match character {
                ' ' => {}
                ']' => {}
                '\n' => {}
                '[' => {}
                _ => {
                    let column = index/4;
                    let new_crate = Crate::Filled(character);
                    stacks[column].push_back(new_crate);
                }
            }
        })
    });
    stacks.iter().for_each(|stack|{
        println!("{:?}",stack);
    });
    stacks
}

fn answer_one<'a>(input: &str) -> String {
    let (top_half, bottom_half) = input.split_at(input.find("\n\n").unwrap());
    let bottom_half = bottom_half.trim();
    let mut crates = get_crates(top_half);
    let moves = get_moves(bottom_half);
    format!("{:?}", moves);

    moves.iter().for_each(|current_move|{
        for x in 0..current_move.count {
            let mut current : Crate = crates[current_move.origin-1].pop_back().unwrap();
            crates[current_move.destination-1].push_back(current);

            println!("Updated:");
            crates.iter().for_each(|stack|{
                println!("{:?}",stack);
            });
        }
    });
    let mut top_of_stack = String::new();
    crates.iter().for_each(|crate_stack|{
        match crate_stack.back(){
            None => {
                top_of_stack.push(' ');
            }
            Some(crate_contents) => {
                match crate_contents {
                    Crate::Empty => {
                        top_of_stack.push(' ');
                    }
                    Crate::Filled(contents) => {
                        top_of_stack.push(*contents);
                    }
                }
            }
        }
    });
    top_of_stack
}

fn answer_two(input: &str) -> String {
    let (top_half, bottom_half) = input.split_at(input.find("\n\n").unwrap());
    let bottom_half = bottom_half.trim();
    let mut crates = get_crates(top_half);
    let moves = get_moves(bottom_half);
    format!("{:?}", moves);

    moves.iter().for_each(|current_move|{
        let mut current : VecDeque<Crate> = Default::default();
        for x in 0..current_move.count {
            current.push_front(crates[current_move.origin-1].pop_back().unwrap());
        }
        crates[current_move.destination-1].append(&mut current);
        println!("Updated:");
        crates.iter().for_each(|stack|{
            println!("{:?}",stack);
        });
    });
    let mut top_of_stack = String::new();
    crates.iter().for_each(|crate_stack|{
        match crate_stack.back(){
            None => {
                top_of_stack.push(' ');
            }
            Some(crate_contents) => {
                match crate_contents {
                    Crate::Empty => {
                        top_of_stack.push(' ');
                    }
                    Crate::Filled(contents) => {
                        top_of_stack.push(*contents);
                    }
                }
            }
        }
    });
    top_of_stack
}

//This is nicer test syntax.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_test_input() {
        assert_eq!(
            "CMZ",
            answer_one(
                include_str!("../input/test_input.txt")
            )
        );
    }

    #[test]
    fn test_part_1_test_input_moves() {
        let input = include_str!("../input/test_input.txt");
        let (top_half, bottom_half) = input.split_at(input.find("\n\n").unwrap());
        let bottom_half = bottom_half.trim();
        println!("{}", bottom_half);
        let moves = get_moves(bottom_half);
        assert_eq!(moves.len(), 4);
        assert_eq!(moves.len(), bottom_half.lines().count());
        println!("{:?}", moves);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            "FCVRLMVQP",
            answer_one(
                include_str!("../input/input.txt")
            )
        );
    }

    #[test]
    fn test_part_2_test_input() {
        assert_eq!(
            "MCD",
            answer_two(include_str!("../input/test_input.txt")
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "RWLWGJGFD",
            answer_two(include_str!("../input/input.txt")
            )
        );
    }
}
