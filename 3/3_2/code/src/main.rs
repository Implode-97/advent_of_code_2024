use std::cmp;
use std::fs::read_to_string;

#[derive(PartialEq)]
enum ParseState {
    Do,
    Dont,
}

const MUT: &str = "mul(,)";
const DO: &str = "do()";
const DONT: &str = "don't()";

fn main() {
    let mut state_parser: ParseState = ParseState::Do;
    let mut text = read_to_string("../data/test.txt").unwrap();
    let mut catch_str = "".to_string();
    let mut text_iter = text.chars().into_iter();
    let mut current_char = text_iter.next();
    let mut catch_len = catch_str.len();

    for 





    // while current_char != None {
    //     if state_parser == ParseState::Do {
    //         if (catch_str.chars().nth(0) == Some('m')) {
    //             if MUT.chars().nth(catch_len).unwrap() == current_char.unwrap() {
    //                 catch_str = catch_str + &current_char.unwrap().to_string();
    //                 catch_len += 1;
    //                 println!("{catch_str}");
    //                 if current_char.unwrap() == ')' {
    //                     println!("calculate shit!");
    //                     catch_str = "".to_string();
    //                     catch_len = 0;
    //                 }
    //             } else if current_char.unwrap().is_numeric() & ((catch_len == 4) | (catch_len == 5))
    //             {
    //                 catch_str = catch_str + &current_char.unwrap().to_string();
    //                 println!("{catch_str}");
    //                 if current_char.unwrap() == ',' {}
    //             }
    //         } else if catch_str.chars().nth(0) == Some('d') {
    //             if DONT.chars().nth(catch_len).unwrap() == current_char.unwrap() {}
    //         }
    //     } else {
    //         current_char = text_iter.next();
    //     }
    // }
}
