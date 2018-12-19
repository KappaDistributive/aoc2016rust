#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::char;

const INPUT: &str = include_str!("../input.txt");
const INPUT_SAMPLE: &str = include_str!("../input_sample.txt");

fn isolate_checksum(input: &str) -> String {
    lazy_static! {
        static ref RE_CHECKSUM: Regex = Regex::new(r"\[(?P<checksum>[a-z]+)\]").unwrap();
    }
    match RE_CHECKSUM.captures(input) {
        Some(cap) => {
      	    return String::from(cap.name("checksum").map_or("", |m| m.as_str()));
        }
        None => {
            panic!("Couldn't retrieve checksum from: {}", input);
        }
    }
}

fn calculate_checksum(input: &str) -> String {
    let mut checksum: String = String::new();
    let mut room: String = isolate_room(input);
    let mut occurance: [u32;26] = [0;26];
    for c in room.chars() {
        occurance[(c as u8 - 'a' as u8) as usize] += 1;
    }
    for _ in 0..5 {
        let mut index: usize = 0;
        let mut max: u32 = 0;
        for i in 0..26 {
            if occurance[i] > max {
                max = occurance[i];
                index = i;
            }
        }
        checksum.push(char::from('a' as u8 + index as u8));
        occurance[index] = 0;
    }
    checksum
}

fn isolate_room(input: &str) -> String {
    lazy_static! {
        static ref RE_ROOM: Regex = Regex::new(r"(?P<room>[\-a-z0-9]+)\[").unwrap();
        static ref RE_ALPHA: Regex = Regex::new(r"[a-z]").unwrap();
    }
    match RE_ROOM.captures(input) {
        Some(cap) => {
            let mut result: String = String::new();
            for c in RE_ALPHA.captures_iter(cap.name("room").map_or("", |m| m.as_str())) {
                result.push_str(c.get(0).unwrap().as_str());
            }
            result
        }
        None => {
            panic!("Couldn't retrieve room from: {}", input);
        }
    }
}

fn isolate_sector_id(input: &str) -> u32 {
    lazy_static! {
        static ref RE_SECTOR_ID: Regex = Regex::new(r"(?P<sector_id>[0-9]+)").unwrap();
    }
    match RE_SECTOR_ID.captures(input) {
        Some(cap) => {
      	    return cap.name("sector_id").map_or(0u32, |m| m.as_str().parse::<u32>().unwrap());
        }
        None => {
            panic!("Couldn't retrieve sector ID from: {}", input);
        }
    }
}

fn is_real(input: &str) -> bool {
    let checksum = isolate_checksum(input);
    let room = isolate_room(input);
    checksum == calculate_checksum(input)
}

fn solve_part_1(input_str: &str) -> u32 {
    let mut result: u32 = 0;
    for line in input_str.trim().lines() {
        if is_real(line) {
            result += isolate_sector_id(line);
        }
    }
    result
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolate_room() {
        assert_eq!("aaaaabbbzyx", isolate_room("aaaaa-bbb-z-y-x-123[abxyz]"));
        assert_eq!("abcdefgh", isolate_room("a-b-c-d-e-f-g-h-987[abcde]"));
    }

    #[test]
    fn test_isolate_checksum() {
        assert_eq!("abxyz", isolate_checksum("aaaaa-bbb-z-y-x-123[abxyz]"));
        assert_eq!("abcde", isolate_checksum("a-b-c-d-e-f-g-h-987[abcde]"));
    }
}
