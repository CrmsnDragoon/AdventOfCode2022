use std::ops::Index;

fn main() {
    println!(
        "answer one: {}",
        detect_overlapping_pairs(
            String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap()
        )
    );
}

#[test]
fn test_input() {
    assert_eq!(
        7,
        detect_overlapping_pairs(
            String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap()
        )
    );
    //assert_eq!(result2, 5);
}

fn detect_overlapping_pairs(input: String) -> i32 {
    let mut overlapping_pair = 0;
    let pairs = input
        .lines()
        .map(|line| {
            let elves = line
                .split(',')
                .map(|assignment| {
                    let rooms = assignment
                        .split('-')
                        .map(|room| room.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    println!("rooms: {}", rooms.len());
                    let start: i32 = *rooms.index(0);
                    let end: i32 = *rooms.index(1);
                    (start, end)
                })
                .collect::<Vec<(i32, i32)>>();
            println!("elves: {}", elves.len());
            let elf1 = elves.index(0);
            let elf2 = elves.index(1);
            (*elf1, *elf2)
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();
    println!("pairs: {}", pairs.len());
    pairs.iter().for_each(|(elf1, elf2)| {
        if elf1.0 <= elf2.0 && elf1.1 >= elf2.1 {
            overlapping_pair += 1;
        }
        if elf1.0 >= elf2.0 && elf1.1 <= elf2.1 {
            overlapping_pair += 1;
        }
    });
    overlapping_pair
}
