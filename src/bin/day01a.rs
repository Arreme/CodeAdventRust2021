use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut increased:u16 = 0;
    if let Ok(lines) = read_lines("inputs/day01input.txt"){
        let mut lines_iter = lines.into_iter();
        let mut last_number = lines_iter.next().unwrap().unwrap().parse::<u16>().unwrap();
        for line in lines_iter {
            let current = line.unwrap().parse::<u16>().unwrap();
            if current > last_number {
                increased += 1;
            }
            last_number = current;
        }
    }
    println!("Total of increased numbers: {}",increased);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> , {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}