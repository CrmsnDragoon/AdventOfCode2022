fn main() {
    let input = include_bytes!("../input/input.txt");
    let input_as_string = String::from_utf8_lossy(input).to_string();
    calc_elf_kcal(input_as_string.clone());
}

fn calc_elf_kcal(input_as_string: String){
    let mut current_elf = 0;

    let mut elves :Vec::<i32> = vec![];

    for current_line in input_as_string.lines() {
        let as_int :Result<i32,_> =  current_line.parse();
        if as_int.is_ok(){
            let current = as_int.unwrap();
            current_elf += current;
        }
        else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }
    elves.sort();
    elves.reverse();
    println!("Answer 1: {}", &elves.first().unwrap());

    let top_3_slice = &elves[0..3];
    println!("top 3: ");
    for x in (&top_3_slice).iter() {
        println!("{}", x);
    }
    println!("Answer 2: top 3 total: {}", top_3_slice.iter().sum::<i32>());
}