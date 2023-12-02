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


fn answer_one(_input: String) -> i32 {
    0
}

fn answer_two(_input: String) -> i32 { 0 }

//This is nicer test syntax.
#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn test_part_1() {
        assert_eq!(
            7,
            answer_one(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"))
        );
        assert_eq!(5, answer_one(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
        assert_eq!(6, answer_one(String::from("nppdvjthqldpwncqszvftbrmjlhg")));
        assert_eq!(
            10,
            answer_one(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
        );
        assert_eq!(
            11,
            answer_one(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))
        );
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn test_part_2() {
        assert_eq!(
            7,
            answer_two(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"))
        );
        assert_eq!(5, answer_two(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")));
        assert_eq!(6, answer_two(String::from("nppdvjthqldpwncqszvftbrmjlhg")));
        assert_eq!(
            10,
            answer_two(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
        );
        assert_eq!(
            11,
            answer_two(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))
        );
    }
}
