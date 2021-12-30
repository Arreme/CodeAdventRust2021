use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main()
{
    let mut input_strings:Vec<String> = Vec::new();
    if let Ok(file) = read_file("inputs/day03inputs.txt")
    {
        for line in file 
        {
            input_strings.push(line.unwrap());
        }
    }

    let mut i = 0:usize;
    let mut final_num:Vec<char> = Vec::new();
/*    while i < 5 {
        let mut zero_count = 0:u16;
        let iter_string = input_strings.iter();
        for input_string in iter_string {
            let j = 0:usize;
            let check_number = true;
            while j < i {

            }
            if check_number {
                if input_string.chars().nth(i).unwrap() == "0" {
                    zero_count += 1;
                } else {
                    zero_count -= 1;
                }
            }

            if zero_count >= 0 {

            }
        }
        i += 1;
    }*/


}

fn bin_to_dec (mut input: Vec<u8>,length: usize, number: u8) -> u32
{
    if input.len() == 0 { return 0; }
    let current = input[length-1] as u32 * 2_u32.pow(number as u32);
    input.truncate(length-1);
    return current + bin_to_dec(input,length-1,number+1);

}

fn read_file<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P : AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}