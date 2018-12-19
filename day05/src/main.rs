extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &str = &"wtnhxymk";

fn solve_part_1(input_str: &str) -> String {
    let mut hasher: Md5 = Md5::new();
    let mut result: String = String::new();
    let key = input_str.as_bytes();
    
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        let mut output: String = String::new();
        output = hasher.result_str();
        if output[0..5] == String::from("00000") {
            result.push_str(&output[5..6]);
            if result.len() == 8 {
                return result
            }
        }
        hasher.reset();
    }
    String::from("")
}

fn solve_part_2(input_str: &str) -> String {
    let mut hasher: Md5 = Md5::new();
    let mut characters: [char;8] = ['_';8];
    let mut progress: u8 = 0;
    let key = input_str.as_bytes();
    
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        let mut output: String = String::new();
        output = hasher.result_str();
        if output[0..5] == String::from("00000") {
            let pos: usize = usize::from_str_radix(&output[5..6], 16).unwrap();
            let character: char = output.as_bytes()[6] as char;
            if pos < 8 && characters[pos] == '_' {
                characters[pos] = character;
                progress += 1;
                if progress == 8 {
                    let mut result: String = String::new();
                    for j in 0..8 {
                        result.push(characters[j]);
                    }
                    return result;
                }
            }
        }
        hasher.reset();
    }
    unreachable!();
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
