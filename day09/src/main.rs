
const INPUT: &str = include_str!("../input.txt");

fn format_input(input_str: &str) -> Vec<char> {
    input_str.chars().collect::<Vec<char>>()
}

fn decompress(input: Vec<char>) -> String {
    let mut result: String = String::new();
    let mut index: usize = 0;
    while index < input.len() {
        if input[index] == '(' {
            index += 1;
            let mut len: String = String::new();
            while input[index] != 'x' {
                len.push(input[index]);
                index += 1;
            }
            let len = len.parse::<usize>().unwrap();
            
            index += 1;
            let mut reps: String = String::new();
            while input[index] != ')' {
                reps.push(input[index]);
                index += 1;
            }
            let reps = reps.parse::<usize>().unwrap();

            index += 1; 
            let mut section: String = String::new();
            for i in index..index+len {
                section.push(input[i]);
            }
            result += &unfold(&section, reps);
            index += len;
        }
        else {
            result.push(input[index]);
            index += 1;
        }
    }
    result
}

fn unfold(section: &String, reps: usize) -> String {
    let mut result: String = String::new();
    for _ in 0..reps {
        result += &section;
    }
    result
}

fn solve_part_1(input_str: &str) -> usize {
    println!("{}",decompress(format_input(input_str)));
    // I don't know why [..].len() is off by one here
    decompress(format_input(input_str)).len() - 1
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decompress() {
        let mut input: String = String::from("ADVENT");
        assert_eq!(decompress(format_input(&input)), String::from("ADVENT"));

        input =  String::from("A(1x5)BC");
        assert_eq!(decompress(format_input(&input)), String::from("ABBBBBC"));

        input =  String::from("(3x3)XYZ");
        assert_eq!(decompress(format_input(&input)), String::from("XYZXYZXYZ"));

        input =  String::from("A(2x2)BCD(2x2)EFG");
        assert_eq!(decompress(format_input(&input)), String::from("ABCBCDEFEFG"));

        input =  String::from("(6x1)(1x3)A");
        assert_eq!(decompress(format_input(&input)), String::from("(1x3)A"));

        input =  String::from("X(8x2)(3x3)ABCY");
        assert_eq!(decompress(format_input(&input)), String::from("X(3x3)ABC(3x3)ABCY"));
    }
}
