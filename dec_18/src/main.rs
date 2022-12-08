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

fn answer_one(input: String) -> i32 {
    0
}

fn answer_two(input: String) -> i32 {
    0
}

//This is nicer test syntax.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            10,
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
