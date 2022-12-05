#![feature(exclusive_range_pattern)]
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    println!(
        "answer one: {}",
        answer_one(String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap())
    );
    println!(
        "answer two: {}",
        answer_two(String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap())
    );
}

fn answer_one<'a>(input: String) -> String {
    let cleaned: String = input
        .chars()
        .filter(|&character| character != '\r')
        .collect();
    let input_parts = cleaned.split("\n\n").collect::<Vec<&str>>();
    let mut part_iter = input_parts.iter();
    let mut initial_state: HashMap<(usize, usize), char> = HashMap::<(usize, usize), char>::new();
    let mut max_row = 0;
    let mut row = 0;
    let mut max_column = 0;
    part_iter.next().unwrap().lines().for_each(|line| {
        let mut column = 0;
        println!(" Reset column: {}", column);
        let mut previous_char: char = '/';
        line.chars().for_each(|character| {
            print!("character: {}", character);
            match character {
                'A'..='Z' => {
                    initial_state.insert((column, row), character);
                    print!(" New Crate: {},{} {}", column, row, character);
                    column += 1;
                    print!(" Increment Column: {}", column);
                    max_column = max_column.max(column);
                }
                ' ' => {
                    if previous_char == ' ' || previous_char == ']' {
                        print!(" Ignoring space");
                    } else {
                        column += 1;
                        print!(" Increment Column: {}", column);
                    }
                }
                '[' => {
                    print!(" Ignoring");
                }
                ']' => {
                    print!(" Ignoring");
                }
                '\n' => {
                    column += 1;
                    print!(" Increment Column: {}", column);
                    max_column = max_column.max(column);
                    max_row = max_row.max(row);
                    row = 0;
                }
                _ => {
                    print!(" Ignoring");
                }
            }
            previous_char = character;
            println!();
        });
        row += 1;
        print!(" Increment row: {}", row);
        max_column = max_column.max(column);
        max_row = max_row.max(row);
        println!();
    });

    println!("num crates: {}", initial_state.len());
    //X, Y, How many to move
    let moves = part_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let numbers = line
                .split(' ')
                .filter_map(|word| match word {
                    "move" => None,
                    "from" => None,
                    "to" => None,
                    _ => Some(word.parse::<i32>().unwrap()),
                })
                .collect::<Vec<i32>>();
            (numbers[0], numbers[1], numbers[2])
        })
        .collect::<Vec<(i32, i32, i32)>>();

    let mut stacks: Vec<VecDeque<char>> = vec![];
    for _ in 0..=max_column {
        stacks.push(VecDeque::<char>::with_capacity(max_row + 1));
    }
    //Need to sort before insertion, I think, otherwise the stacks are going to be backwards.
    for y in 0..=max_row {
        for x in 0..=max_column {
            if initial_state.get(&(x, y)).is_some() {
                stacks[x].push_back(*initial_state.get(&(x, y)).unwrap());
            };
        }
    }

    for _move_command in moves {}

    let mut answer: String = String::new();

    for stack in 0..=max_column {
        println!("{}", stacks[stack].iter().collect::<String>());
        let top = stacks[stack].front();
        if top.is_some() {
            let value = *top.unwrap();
            println!("{} top: {}", stack, &value);
            answer.push(value);
        }
    }

    answer
}

fn answer_two(_input: String) -> i32 {
    0
}

//This is nicer test syntax.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            "CMZ",
            answer_one(
                String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap()
            )
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            20,
            answer_two(
                String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap()
            )
        );
    }
}
