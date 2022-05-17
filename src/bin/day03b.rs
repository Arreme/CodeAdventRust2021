use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// struct PrintableVec<T:Display>(pub Vec<T>);
//
// impl <T:Display> Display for PrintableVec<T>{
//     fn fmt(&self, f: &mut Formatter<>) -> fmt::Result {
//         self.0.iter().fold(Ok(()), |result, content| {
//             return result.and_then(|_| writeln!(f,"{}",content));
//         })
//     }
// }

fn main() {
    let mut string_inputs:Vec<String> = Vec::new();
    if let Ok(file) = read_file("inputs/day03inputs.txt") {
        for line in file {
            string_inputs.push(line.expect("That is not a valid input!"));
        }
    }

    let oxygen_current = oxygen(string_inputs.clone());
    let co2_current = co2_scrubber(string_inputs);
    println!("{}",  to_dec(&oxygen_current[0]) * to_dec(&co2_current[0]));


}

fn oxygen(string_inputs: Vec<String>) -> Vec<String> {
    let length = string_inputs[0].len();
    let mut oxygen_current = string_inputs;

    for i in 0..length {
        if oxygen_current.len() <= 1 {
            break;
        }

        let mut zero_strings: Vec<String> = Vec::new();
        let mut one_strings: Vec<String> = Vec::new();

        for curr_string in oxygen_current {
            let ch = curr_string.chars().nth(i).expect("Not a valid input character!");
            if ch == '0' {
                zero_strings.push(curr_string);
            } else {
                one_strings.push(curr_string);
            }
        }

        if one_strings.len() >= zero_strings.len() {
            oxygen_current = one_strings;
        } else {
            oxygen_current = zero_strings;
        }
    }
    oxygen_current
}

fn co2_scrubber(string_inputs: Vec<String>) -> Vec<String> {
    let length = string_inputs[0].len();
    let mut co2_current = string_inputs;

    for i in 0..length {
        if co2_current.len() <= 1 {
            break;
        }

        let mut zero_strings: Vec<String> = Vec::new();
        let mut one_strings: Vec<String> = Vec::new();

        for curr_string in co2_current {
            let ch = curr_string.chars().nth(i).expect("Not a valid input character!");
            if ch == '0' {
                zero_strings.push(curr_string);
            } else {
                one_strings.push(curr_string);
            }
        }

        if one_strings.len() < zero_strings.len() {
            co2_current = one_strings;
        } else {
            co2_current = zero_strings;
        }
    }
    co2_current
}

fn to_dec(val: &str) -> u32 {
    return u32::from_str_radix(val, 2).unwrap();
}

fn read_file<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P : AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}