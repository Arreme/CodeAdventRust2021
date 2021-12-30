use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Point {
    pub x:i32,
    pub y:i32,
    pub aim: i32,
}

fn main() {
    let mut position = Point {x: 0, y: 0, aim: 0};
    if let Ok(file) = read_lines("inputs/day02inputs.txt"){
        for line in file {
            let result = line.unwrap();
            let command: Vec<&str> = result.split(" ").collect();
            match command[0] {
                "forward" => {
                    position.x += command[1].parse::<i32>().unwrap();
                    position.y += command[1].parse::<i32>().unwrap() * position.aim;
                },
                "up" => position.aim -=  command[1].parse::<i32>().unwrap(),
                "down" => position.aim +=  command[1].parse::<i32>().unwrap(),
                _ => println!("Don't know this")
            }
        }
    }

    println!("Final position {:?}",position);
    println!("Multiplied {}: ",position.x * position.y);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>,  {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}