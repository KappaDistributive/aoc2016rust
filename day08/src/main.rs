#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

struct Display {
    screen: [[u8;50];6],
}

impl Display {    
    fn new() -> Self{
        Display{ screen: [[0u8;50];6] }
    }

    fn execute(&mut self, input: &str) {
        lazy_static!{
            static ref RE_DIGITS: Regex = Regex::new(r"(?P<left>[0-9]+)[^0-9]+(?P<right>[0-9]+)").unwrap();
        }
        match RE_DIGITS.captures(input) {
            Some(cap) => {
                let left = cap.name("left").map_or(0, |m| m.as_str().parse::<usize>().unwrap());
                let right = cap.name("right").map_or(0, |m| m.as_str().parse::<usize>().unwrap());
                if input.contains("rect") {
                    self.rect(left, right);
                    //println!("rect: {}x{}", left, right);
                }
                else if input.contains("y=") {
                    self.rotate_row(right, left);
                    //println!("rotate row y={}, by {}", left, right);
                }
                else if input.contains("x=") {
                    self.rotate_column(left, right);
                    //println!("rotate column x={}, by {}", left, right);
                }
                else {
                    panic!("Couldn't execute: {}", input);
                }
            }
            None => {
                panic!("Couldn't execute: {}", input);
            }
        }
        
    }
    
    fn rect(&mut self, x: usize, y: usize) {
        for j in 0..y {
            for i in 0..x {
                self.screen[j][i] = 1;
            }
        }
    }

    fn rotate_column(&mut self, x: usize, y: usize) {
        let mut temp: Vec<u8> = Vec::new();
        for i in 0..6 {
            temp.push(self.screen[i][x]);            
        }
        for i in 0..6 {
            self.screen[(i + y) % 6][x] = temp[i];
        }
    }

    fn rotate_row(&mut self, x: usize, y: usize) {
        let mut temp: Vec<u8> = Vec::new();
        for i in 0..50 {
            temp.push(self.screen[y][i]);            
        }
        for i in 0..50 {
            self.screen[y][(i + x) % 50] = temp[i];
        }
    }

    fn print(&self) {
        for y in 0..6 {
            for x in 0..50 {
                match self.screen[y][x] {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => unreachable!(),
                }
            }
            println!();
        }
    }

    fn lit_pixels(&self) -> usize {
        let mut result: usize = 0;
        for y in 0..6 {
            for x in 0..50 {
                if self.screen[y][x] == 1 {
                    result += 1;
                }
            }
        }
        result
    }
}

fn solve_part_1(input_str: &str) -> usize {
    let mut d: Display = Display::new();
    for line in input_str.lines() {
        d.execute(line);
    }
    d.lit_pixels()
}

fn solve_part_2(input_str: &str) {
    let mut d: Display = Display::new();
    for line in input_str.lines() {
        d.execute(line);
    }
    d.print()
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2:");
    solve_part_2(INPUT);
}
