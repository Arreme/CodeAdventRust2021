use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut final_num:Vec<u8> = Vec::new();

    for i in 0..12 {
        let mut occ = 0;
        if let Ok(file) = read_file("inputs/day03inputs.txt") {
            for line in file {
                let hi_occurrences:Vec<char> = line.unwrap().chars().collect();
                if hi_occurrences[i] == '1' { occ += 1; }
            }
        }

        if occ > 500 {final_num.push(1) } else { final_num.push(0) }
    }

    println!("Final num: {:?}",final_num);
    let inverted_num:Vec<u8> = final_num.iter().map(|x|{if *x==1 { 0 } else { 1 }}).collect();
    println!("Inverted num: {:?}",inverted_num);
    let length = final_num.len();
    let gamma = bin_to_dec(final_num,length,0);
    let beta = bin_to_dec(inverted_num,length,0);
    println!("Final: {}",gamma*beta);
}

fn bin_to_dec (mut input: Vec<u8>,length: usize, number: u8) -> u32
{
    if input.len() == 0 { return 0; }
    let current = input[length-1] as u32 * 2_u32.pow(number as u32);
    input.truncate(length-1);
    return current + bin_to_dec(input,length-1,number+1);

}

fn read_file<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P : AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}