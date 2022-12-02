fn main() {
    let result1 = roshambo_all_rounds(String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap());
    println!("answer one: {}", result1);
    let result2 = roshambo_all_rounds_strat_guide(String::from_utf8(include_bytes!("../input/input.txt").to_vec()).unwrap());
    println!("answer two: {}", result2);
}


#[test]
fn test_input() {
    let input = String::from_utf8(include_bytes!("../input/test_input.txt").to_vec()).unwrap();
    let round_input = input.clone();
    let mut iter = round_input.lines();
    assert_eq!(roshambo_round(iter.next().unwrap().to_string()),8);
    assert_eq!(roshambo_round(iter.next().unwrap().to_string()),1);
    assert_eq!(roshambo_round(iter.next().unwrap().to_string()),6);

    let result1 = roshambo_all_rounds(input.clone());
    assert_eq!(result1, 15);
    let round_input = input.clone();
    let mut iter = round_input.lines();
    assert_eq!(roshambo_strat_round(iter.next().unwrap().to_string()),4);
    assert_eq!(roshambo_strat_round(iter.next().unwrap().to_string()),1);
    assert_eq!(roshambo_strat_round(iter.next().unwrap().to_string()),7);
    assert_eq!(roshambo_strat_round("A X".to_string()),0+3);
    assert_eq!(roshambo_strat_round("A Y".to_string()),3+1);
    assert_eq!(roshambo_strat_round("A Z".to_string()),6+2);
    let result2 = roshambo_all_rounds_strat_guide(input);
    assert_eq!(result2, 12);
}

fn roshambo_all_rounds(input: String)->i32{
    input.lines().map(|line|{
        roshambo_round(line.to_string())
    }).sum::<i32>()
}
fn roshambo_round(line: String) ->i32{
    let mut moves = line.split(' ');
    let opponent = moves.next().unwrap();
    let me = moves.next().unwrap();
    let mut current_score = match me.chars().next().unwrap() {
        'X'=>{
            println!("Me: ROCK!");
            1
        },
        'Y'=>{
            println!("Me: PAPER!");
            2
            },
        'Z'=>{
            println!("Me: SCISSORS!");
            3
                },
        _=>{
            println!("ERROR!");
            0
        }
    };
    match opponent.chars().next().unwrap(){
        'A'=>{
            println!("THEM: ROCK!");
            if current_score == 1{current_score += 3}
            if current_score == 2{current_score += 6}
        }
        'B'=>{
            println!("THEM: PAPER!");
            if current_score == 2{current_score += 3}
            if current_score == 3{current_score += 6}
        }
        'C'=>{
            println!("THEM: SCISSORS!");
            if current_score == 3{
                current_score += 3
            }
            if current_score == 1{
                current_score += 6
            }
        }
        _=>{
            println!("ERROR!")
        }
    }
    println!("current_score: {}",&current_score);
    current_score
}
fn roshambo_strat_round(line: String) ->i32{
    println!("{}",line);
    let mut moves = line.split(' ');
    let opponent = moves.next().unwrap().chars().next().unwrap();
    println!("Opponent: {}", opponent);
    let me = moves.next().unwrap().chars().next().unwrap();
    println!("Me: {}", me);
    let mut current_score = match me {
        'X'=>{
            println!("Me: LOSE!");
            0
        },
        'Y'=>{
            println!("Me: DRAW!");
            3
        },
        'Z'=>{
            println!("Me: WIN!");
            6
        },
        _=>{
            println!("ERROR!");
            0
        }
    };
    match opponent{
        'A'=>{
            println!("THEM: ROCK!");
            if current_score == 0{
                println!("I MUST LOOSE! SCISSORS!");
                current_score += 3
            }//Lose
            else if current_score == 3{
                println!("I MUST DRAW! ROCK!");
                current_score += 1
            }//Draw
            else if current_score == 6{
                println!("I MUST WIN! PAPER!");
                current_score += 2
            }//Win
        }
        'B'=>{
            println!("THEM: PAPER!");
            if current_score == 0{
                println!("I MUST LOOSE! ROCK!");
                current_score += 1
            }
            else if current_score == 3{
                println!("I MUST DRAW! PAPER!");
                current_score += 2
            }
            else if current_score == 6{
                println!("I MUST WIN! SCISSORS!");
                current_score += 3
            }
        }
        'C'=>{
            println!("THEM: SCISSORS!");
            if current_score == 0{
                println!("I MUST LOSE! PAPER!");
                current_score += 2
            }
            else if current_score == 3{
                println!("I MUST DRAW! SCISSORS!");
                current_score += 3
            }
            else if current_score == 6{
                println!("I MUST WIN! ROCK!");
                current_score += 1
            }
        }
        _=>{
            println!("ERROR!")
        }
    }
    println!("current_score: {}",&current_score);
    current_score
}

fn roshambo_all_rounds_strat_guide(input: String)->i32{
    input.lines().map(|line|{
        roshambo_strat_round(line.to_string())
    }).sum::<i32>()
}