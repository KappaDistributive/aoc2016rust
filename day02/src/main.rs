
const INPUT: &str = include_str!("../input.txt");

fn solve_part_1(input_str: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut pos: (usize,usize) = (1,1);
    let keypad = [["1","2","3"],
                  ["4","5","6"],
                  ["7","8","9"]];
    for line in input_str.lines() {
        for c in line.chars() {
            match c {
                'L' => {
                    if pos.0 > 0 {
                        pos.0 -= 1;
                    }
                }
                'R' => {
                    if pos.0 < 2 {
                        pos.0 += 1;
                    }
                }
                'U' => {
                    if pos.1 > 0 {
                        pos.1 -= 1;
                    }
                }
                'D' => {
                    if pos.1 < 2 {
                        pos.1 += 1;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
        result.push(keypad[pos.1][pos.0]);
    }
    result
}

fn solve_part_2(input_str: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut pos: (usize,usize) = (1,1);
    let keypad = [[" "," ","1"," "," "],
                  [" ","2","3","4"," "],
                  ["5","6","7","8","9"],
                  [" ","A","B","C"," "],
                  [" "," ","D"," "," "]];
    
    for line in input_str.lines() {
        for c in line.chars() {
            match c {
                'L' => {
                    if pos.0 > 0 && keypad[pos.1][pos.0 - 1] != " " {
                        pos.0 -= 1;
                    }
                }
                'R' => {
                    if pos.0 < 4 && keypad[pos.1][pos.0 + 1] != " " {
                        pos.0 += 1;
                    }
                }
                'U' => {
                    if pos.1 > 0 && keypad[pos.1 - 1][pos.0] != " " {
                        pos.1 -= 1;
                    }
                }
                'D' => {
                    if pos.1 < 4 && keypad[pos.1 + 1][pos.0] != " " {
                        pos.1 += 1;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
        result.push(keypad[pos.1][pos.0]);
    }
    result
}

fn main() {
    let mut answer_1: String = String::new();
    for c in solve_part_1(INPUT) {
        answer_1.push_str(c);
    }
    println!("Answer part 1: {}", answer_1);
    
    let mut answer_2: String = String::new();
    for c in solve_part_2(INPUT) {
        answer_2.push_str(c);
    }
    println!("Answer part 2: {}", answer_2);
}
