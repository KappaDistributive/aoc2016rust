extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
const INPUT: &str = include_str!("../input.txt");

fn is_abba(input: [char; 4]) -> bool {
    let [a, b, c, d] = input;
    a != b && a == d && b == c
}

fn contains_abba(input: &Vec<char>) -> bool {
    for i in 0..input.len() - 3 {
        let temp: [char; 4] = [input[i], input[i + 1], input[i + 2], input[i + 3]];
        if is_abba(temp) {
            return true;
        }
    }
    false
}

fn supports_tlp(input: &Vec<char>) -> bool {
    let mut input_string: String = String::new();
    for c in input.clone() {
        input_string.push(c);
    }
    lazy_static! {
        static ref RE_HYPERNET: Regex = Regex::new(r"(?P<hypernet>\[[a-z]+\])").unwrap();
    }
    for c in RE_HYPERNET.captures_iter(&input_string) {
        if contains_abba(&c["hypernet"].chars().collect::<Vec<char>>()) {
            return false;
        }
    }
    contains_abba(input)
}

fn solve_part_1(input_str: &str) -> usize {
    let mut result: usize = 0;
    for line in input_str.lines() {
        if supports_tlp(&line.chars().collect::<Vec<char>>()) {
            result += 1;
        }
    }
    result
}
fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
}
