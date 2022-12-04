fn main() {
    let input = include_bytes!("../input/input.txt");
    let input_as_string = String::from_utf8_lossy(input).to_string();
    let res = calc_elf_kcal(input_as_string.clone());
    println!("Answer 1: {}", res.0);
    println!("Answer 2: top 3 total: {}", res.1);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dec01() {
        let input = include_bytes!("../input/test_input.txt");
        let input_as_string = String::from_utf8_lossy(input).to_string();
        let (answer1, answer2) = calc_elf_kcal(input_as_string.clone());
        assert_eq!(answer1, 24000);
        assert_eq!(answer2, 45000);
    }
}
fn calc_elf_kcal(input_as_string: String) -> (i32, i32) {
    let mut current_elf = 0;

    let mut elves: Vec<i32> = vec![];

    for current_line in input_as_string.lines() {
        let as_int: Result<i32, _> = current_line.parse();
        if as_int.is_ok() {
            let current = as_int.unwrap();
            current_elf += current;
        } else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }
    if current_elf != 0 {
        elves.push(current_elf);
    }
    elves.sort();
    elves.reverse();

    let answer1: i32 = elves.first().unwrap().clone();

    let top_3_slice = &elves[0..3];
    let answer2: i32 = top_3_slice.iter().sum::<i32>();
    (answer1, answer2)
}
