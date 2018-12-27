const INPUT: &str = include_str!("../input.txt");

fn format_input(input_str: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for line in input_str.lines() {
        result.push(line.chars().collect::<Vec<char>>());
    }
    result
}

fn solve_part_1(input_str: &str) -> Result<String, String> {
    let received: Vec<Vec<char>> = format_input(input_str);
    let mut counts: Vec<[usize; 26]> = Vec::new();
    let mut index: usize = 0;
    let len = received.clone()[0].len();
    for i in 0..len {
        counts.push([0; 26]);
        for message in received.clone() {
            if message.len() != len {
                return Result::Err(String::from("Couldn't parse input!"));
            }
            counts[index][(message[i] as u8 - 'a' as u8) as usize] += 1;
        }
        index += 1;
    }
    let mut result: String = String::new();
    for i in 0..len {
        let mut max: usize = 0;
        let mut c: u8 = 0;
        for j in 0..26 {
            if counts[i][j] > max {
                max = counts[i][j];
                c = j as u8;
            }
        }
        result.push((c as u8 + 'a' as u8) as char);
    }
    Ok(result)
}

fn solve_part_2(input_str: &str) -> Result<String, String> {
    let received: Vec<Vec<char>> = format_input(input_str);
    let mut counts: Vec<[usize; 26]> = Vec::new();
    let mut index: usize = 0;
    let len = received.clone()[0].len();
    for i in 0..len {
        counts.push([0; 26]);
        for message in received.clone() {
            if message.len() != len {
                return Result::Err(String::from("Couldn't parse input!"));
            }
            counts[index][(message[i] as u8 - 'a' as u8) as usize] += 1;
        }
        index += 1;
    }
    let mut result: String = String::new();
    for i in 0..len {
        let mut min: usize = std::usize::MAX;
        let mut c: u8 = 0;
        for j in 0..26 {
            if counts[i][j] < min && 0 < counts[i][j] {
                min = counts[i][j];
                c = j as u8;
            }
        }
        result.push((c as u8 + 'a' as u8) as char);
    }
    Ok(result)
}

fn main() {
    println!("Answer to part 1: {}", solve_part_1(INPUT).unwrap());
    println!("Answer to part 2: {}", solve_part_2(INPUT).unwrap());
}
