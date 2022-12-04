use std::ops::Index;

fn main() {
    println!(
        "answer one: {}",
        prioritize_backpack(
            String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap()
        )
    );
    println!(
        "answer two: {}",
        get_badges_for_authorization(
            String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap()
        )
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        assert_eq!(
            157,
            prioritize_backpack(
                String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap()
            )
        );
        assert_eq!(
            70,
            get_badges_for_authorization(
                String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap()
            )
        );
    }
}

fn prioritize_backpack(input: String) -> i32 {
    let mut priority_items: Vec<char> = vec![];
    input
        .lines()
        .map(|line| {
            let total_size = line.len();
            let compartment_size = total_size / 2;
            (
                (line.get(0..compartment_size).unwrap().into()),
                (line.get(compartment_size..total_size).unwrap().into()),
            )
        })
        .collect::<Vec<(String, String)>>()
        .iter()
        .for_each(|(compartment1, compartment2)| {
            let mut breakout = false;
            for c1 in compartment1.chars() {
                for c2 in compartment2.chars() {
                    if c1 == c2 {
                        println!(
                            "Found dupe: {}={},i32:{},u8:{}",
                            c1, c2, c2 as i32, c2 as u8
                        );
                        priority_items.push(c1);
                        breakout = true;
                        break;
                    }
                }
                if breakout {
                    break;
                }
            }
        });

    score_items(priority_items)
}

fn score_items(input: Vec<char>) -> i32 {
    input
        .iter()
        .map(|&item| {
            let value;
            let as_u8: u8 = item as u8;
            if as_u8 > '`' as u8 {
                value = as_u8 as i32 - 96;
            } else {
                value = as_u8 as i32 - 64 + 26;
            }
            println!("Scored item: {}={}", item, value);
            value
        })
        .sum::<i32>()
}

fn get_badges_for_authorization(input: String) -> i32 {
    let mut badges: Vec<char> = vec![];
    let groups = input.lines().collect::<Vec<&str>>();
    let groups = groups.chunks(3).into_iter();
    println!("Groups: {}", groups.len());
    groups.for_each(|chunk| {
        let (&bag1, &bag2, &bag3) = (chunk.index(0), chunk.index(1), chunk.index(2));
        let mut breakout = false;
        for c1 in bag1.chars() {
            for c2 in bag2.chars() {
                if c1 == c2 {
                    for c3 in bag3.chars() {
                        if c1 == c3 {
                            println!("Found Item: {} in all bags in the group", c3);
                            badges.push(c3);
                            breakout = true;
                            break;
                        }
                    }
                }
                if breakout {
                    break;
                }
            }
            if breakout {
                break;
            }
        }
    });
    score_items(badges)
}
