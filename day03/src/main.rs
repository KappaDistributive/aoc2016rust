extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");
type Point = (u32,u32,u32);

fn format_input(input_str: &str) -> Vec<Point>{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"\s*([0-9]*)\s*([0-9]*)\s*([0-9]*)").unwrap();
    }
    let mut output:Vec<Point> = Vec::new();
    for line in input_str.trim().lines() {
        match RE.captures(line) {
            Some(caps) => {
                let (a,b,c) = (caps.get(1).map_or(0u32, |m| m.as_str().parse::<u32>().unwrap()),
                               caps.get(2).map_or(0u32, |m| m.as_str().parse::<u32>().unwrap()),
                               caps.get(3).map_or(0u32, |m| m.as_str().parse::<u32>().unwrap()));
                output.push((a,b,c));
            }
            None => {
                panic!("Couldn't parse: {}", line);
            }
        }
    }
    output
}

fn format_input_2(input_str: &str) -> Vec<u32>{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"\s*([0-9]*)\s*([0-9]*)\s*([0-9]*)").unwrap();
    }
    let mut output:Vec<u32> = Vec::new();
    for line in input_str.trim().lines() {
        match RE.captures(line) {
            Some(caps) => {
                let (a,b,c) = (caps.get(1).map_or(0u32, |m| m.as_str().parse::<u32>().unwrap()),
                               caps.get(2).map_or(0u32, |m| m.as_str().parse::<u32>().unwrap()),
                               caps.get(3).map_or(0u32, |m| m.as_str().parse::<u32>().unwrap()));
                output.push(a);
                output.push(b);
                output.push(c);
            }
            None => {
                panic!("Couldn't parse: {}", line);
            }
        }
    }
    output
}

fn solve_part_1(input_str: &str) -> usize {
    let mut triangles: Vec<Point> = Vec::new();
    let candidates: Vec<Point> = format_input(input_str);

    for (a,b,c) in candidates {
        if a + b > c && a + c > b && b + c > a {
            triangles.push((a,b,c));
        }
    }
    triangles.len()
}

fn solve_part_2(input_str: &str) -> usize {
    let mut triangles: Vec<Point> = Vec::new();
    let values: Vec<u32> = format_input_2(input_str);

    let number_candidates: usize = values.len() / 9;

    for i in 0..number_candidates {
        for j in 0..3 {
            let a = values[i*9+0+j];
            let b = values[i*9+3+j];
            let c = values[i*9+6+j];
            if a + b > c && a + c > b && b + c > a {
            triangles.push((a,b,c));
            }
        }
    }
    triangles.len()
}
fn main() {
    println!("Answer part 1: {}",solve_part_1(INPUT));
    println!("Answer part 2: {}",solve_part_2(INPUT));
}
