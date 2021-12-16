use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let mut increased:u16 = 0;
    if let Ok(lines) = read_lines("inputs/day01input.txt"){
        let mut lines_iter = lines.into_iter();
        let mut vec_deq:VecDeque<u16> = VecDeque::with_capacity(3);
        vec_deq.push_back(lines_iter.next().unwrap().unwrap().parse::<u16>().unwrap());
        vec_deq.push_back(lines_iter.next().unwrap().unwrap().parse::<u16>().unwrap());
        vec_deq.push_back(lines_iter.next().unwrap().unwrap().parse::<u16>().unwrap());
        let mut last_sum:u16 = vec_deq.iter().sum();
        for line in lines_iter {
            vec_deq.pop_front();
            vec_deq.push_back(line.unwrap().parse::<u16>().unwrap());
            let current:u16 = vec_deq.iter().sum();
            if current > last_sum {
                increased += 1;
            }
            last_sum = current;
        }
    }
    println!("Total of increased numbers: {}",increased);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> , {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}