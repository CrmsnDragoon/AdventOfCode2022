use std::ops::Index;

fn main() {
    println!(
        "answer one: {}",
        detect_encapsulating_pairs(
            String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap()
        )
    );
    println!(
        "answer two: {}",
        detect_overlapping_pairs(
            String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap()
        )
    );
}

fn detect_encapsulating_pairs(input: String) -> i32 {
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
                    let start: i32 = *rooms.index(0);
                    let end: i32 = *rooms.index(1);
                    (start, end)
                })
                .collect::<Vec<(i32, i32)>>();
            let elf1 = elves.index(0);
            let elf2 = elves.index(1);
            (*elf1, *elf2)
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();
    println!("total pairs: {}", pairs.len());
    pairs.iter().for_each(|(elf1, elf2)| {
        //Assignment encapsulation checks
        if elf1.0 <= elf2.0 && elf1.1 >= elf2.1 {
            overlapping_pair += 1;
        } else if elf1.0 >= elf2.0 && elf1.1 <= elf2.1 {
            overlapping_pair += 1;
        }
    });
    overlapping_pair
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
                    let start: i32 = *rooms.index(0);
                    let end: i32 = *rooms.index(1);
                    (start, end)
                })
                .collect::<Vec<(i32, i32)>>();
            let elf1 = elves.index(0);
            let elf2 = elves.index(1);
            (*elf1, *elf2)
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();
    println!("total pairs: {}", pairs.len());
    pairs.iter().for_each(|(elf1, elf2)| {
        //Apparently I should have used ranges instead, but while it's harder to parse it's probably faster? maybe.
        //Is Elf 2 inside of Elf 1's assignment
        if elf1.0 >= elf2.0 && elf1.0 <= elf2.1 {
            overlapping_pair += 1;
        }
        //Does elf 2 overlap
        else if elf2.0 >= elf1.0 && elf2.0 <= elf1.1 {
            overlapping_pair += 1;
        }
    });
    overlapping_pair
}

//This is nicer test syntax.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            2,
            detect_encapsulating_pairs(
                String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap()
            )
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            4,
            detect_overlapping_pairs(
                String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap()
            )
        );
    }
}
