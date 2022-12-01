use std::string::FromUtf8Error;

fn main() {
    let input = (include_bytes!("../input/input.txt"));
    let input_as_string = String::from_utf8_lossy(input).to_string();
    calc_biggest_elf_kcal(input_as_string);
}

fn calc_biggest_elf_kcal(input_as_string: String){
    let mut current_elf = 0;

    let mut elves :Vec::<i32> = vec![];

    for current_line in input_as_string.lines() {
        let as_int :Result<i32,_> =  current_line.parse();
        if as_int.is_ok(){
            let current = as_int.unwrap();
            println!("{}", current.clone());
            current_elf += current;
        }
        else {
            println!("new elf! total: {}", current_elf);
            elves.push(current_elf);
            current_elf = 0;
        }
    };
    elves.sort();
    println!("{}", elves.last().unwrap());
}