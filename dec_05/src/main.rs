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
        let mut line_iter = line.chars().peekable();
        let mut column = 0;
        loop {
            let current = line_iter.next();
            if current.is_some() {
                let current_char = current.unwrap();
                match current_char {
                    ' ' => {
                        let mut space_count = 0;
                        while line_iter.peek().is_some() && line_iter.peek().unwrap() == &' ' {
                            line_iter.next();
                            space_count += 1;
                            if space_count > 2 {
                                column += 1;
                                space_count -= 3;
                                //eat the next character, it's a space.
                                line_iter.next();
                            }
                        }
                    }
                    '[' => {
                        initial_state.insert((column, row), line_iter.next().unwrap());
                        line_iter.next();
                        column += 1;
                    }
                    _ => {}
                }
            }
            if line_iter.peek().is_none() {
                break;
            }
        }
        row += 1;
        //print!(" Increment row: {}", row);
        max_column = max_column.max(column);
        max_row = max_row.max(row);
        //println!();
    });

    println!(
        "initial state crates: {}",
        initial_state
            .iter()
            .map(|crate_pos| {
                format!(
                    "{:2},{:2}:[{}]\n",
                    crate_pos.0 .0, crate_pos.0 .1, crate_pos.1
                )
            })
            .collect::<String>()
    );
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
                    _ => Some(word.parse::<usize>().unwrap()),
                })
                .collect::<Vec<usize>>();
            println!("move {} from {} to {}", numbers[0], numbers[1], numbers[2]);
            (numbers[0], numbers[1] - 1, numbers[2] - 1)
        })
        .collect::<Vec<(usize, usize, usize)>>();

    let mut stacks: Vec<VecDeque<char>> = vec![];
    for _ in 0..max_column {
        stacks.push(VecDeque::<char>::with_capacity(max_row + 1));
    }
    //Need to sort before insertion, I think, otherwise the stacks are going to be backwards.
    for current_row in 0..=max_row {
        for current_column in 0..=max_column {
            if initial_state.get(&(current_column, current_row)).is_none() {
                continue;
            }
            stacks[current_column]
                .push_back(*initial_state.get(&(current_column, current_row)).unwrap());
        }
    }
    println!("current state:");
    for stack in 0..max_column {
        println!(
            "\t{:2} stack: {}",
            stack + 1,
            stacks[stack].iter().collect::<String>()
        );
    }

    for (crate_count, column_src, column_dest) in moves {
        println!(
            "command: move {} from {} to {}",
            crate_count,
            column_src + 1,
            column_dest + 1,
        );

        for crate_move_count in 0..crate_count {
            let getter = stacks[column_src].pop_front();
            if getter.is_some() {
                let current = getter.unwrap();
                println!(
                    "command: move {} from {} to {}: Moved {}, count: {}/{}",
                    crate_count,
                    column_src + 1,
                    column_dest + 1,
                    &current,
                    crate_move_count + 1,
                    crate_count
                );
                stacks[column_dest].push_front(current);
            } else {
                println!(
                    "Failed to process command {}: move {} from {} to {}, count: {}/{}",
                    crate_move_count,
                    crate_count,
                    column_src + 1,
                    column_dest + 1,
                    crate_move_count + 1,
                    crate_count
                );
                assert!(false)
            }
            println!("current state:");
            for stack in 0..max_column {
                println!(
                    "\t{:2} stack: {}",
                    stack + 1,
                    stacks[stack].iter().collect::<String>()
                );
            }
        }
        println!("current state:");
        for stack in 0..max_column {
            println!(
                "\t{:2} stack: {}",
                stack + 1,
                stacks[stack].iter().collect::<String>()
            );
        }
    }

    let mut answer: String = String::new();

    for stack in 0..max_column {
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
