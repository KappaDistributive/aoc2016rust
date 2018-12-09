extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn format_input(input_str: &str) -> Vec<(char,i32)>{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"([A-Z])([0-9]*)").unwrap();
    }
    let mut commands: Vec<(char,i32)> = Vec::new();    
    for c in input_str.split(", ")  {
        let mut direction: char;
        let mut distance: i32;
        match RE.captures(c) {
            Some(caps) => {
                direction = caps.get(1).map_or(' ',|m| m.as_str().chars().next().unwrap());
                distance = caps.get(2).map_or(0i32, |m| m.as_str().parse::<i32>().unwrap());
            }
            None => {
                panic!("Error while parsing: {}", c);
            }
        }
        commands.push((direction,distance));
    }
    commands
}

fn solve_part_1(input: &str) -> i32 {
    let commands:Vec<(char,i32)> = format_input(input);
    let mut pos: (i32,i32) = (0i32,0i32);
    let mut orientation: u8 = 0u8;
    for (direction,distance) in commands {
        match direction {
            'L' => {
                if orientation == 0 {
                    orientation = 3;
                }
                else {
                    orientation -= 1;
                }
            }
            'R' => {
                orientation = (orientation + 1) % 4;
            }
            _ => {
                unreachable!();
            }
        }
        match orientation {
            0 => {
                pos.0 += distance;
            }
            1 => {
                pos.1 += distance;
            }
            2 => {
                pos.0 -= distance;
            }
            3 => {
                pos.1 -= distance;
            }
            _ => {
                unreachable!();
            }
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn solve_part_2(input: &str) -> i32 {
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    let commands: Vec<(char,i32)> = format_input(input);
    let mut pos: (i32,i32) = (0i32,0i32);
    visited.insert(pos);
    let mut orientation: u8 = 0u8;
    for (direction,distance) in commands {
        match direction {
            'L' => {
                if orientation == 0 {
                    orientation = 3;
                }
                else {
                    orientation -= 1;
                }
            }
            'R' => {
                orientation = (orientation + 1) % 4;
            }
            _ => {
                unreachable!();
            }
        }
        for _ in 0..distance {
            match orientation {
                0 => {
                    pos.0 += 1;
                }
                1 => {
                    pos.1 += 1;
                }
                2 => {
                    pos.0 -= 1;
                }
                3 => {
                    pos.1 -= 1;
                }
                _ => {
                    unreachable!();
                }
            }
            if visited.contains(&pos) {
                return pos.0.abs() + pos.1.abs();
            }
            visited.insert(pos);
        }
    }
    panic!("Never reached a position twice!");
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
