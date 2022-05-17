use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy,Clone)]
struct Cell{
    marked: bool,
    number: u8,
}

struct Board {
    cells: [[Cell ; 5] ; 5],
    hasWon: bool
}

impl Cell {
    fn new(num:u8) -> Self {
        Cell{marked:false,number:num}
    }
}

impl Board {
    fn check_win(&self, i:usize, j:usize) -> bool {
       self.cells.iter().all(|x| x[j].marked) || self.cells[i].iter().all(|x| x.marked)
    }

    fn mark_board(&mut self, num:u8) -> bool{
        for i in 0..5 {
            for j in 0..5 {
                if self.cells[i][j].number == num {
                    self.cells[i][j].marked = true;
                    return self.check_win(i, j);
                }
            }
        }
        return false;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cells.iter().fold(Ok(()), |result, vector| {
            result.and_then(|_| vector.iter().fold(Ok(()),|in_result,cell| {
                if cell.marked {
                    in_result.and_then(|_| write!(f,"\x1b[91m({:0>2})\x1b[0m",cell.number))
                } else {
                    in_result.and_then(|_| write!(f,"({:0>2})",cell.number))
                }
            })).and_then(|_| writeln!(f,"{}",""))
        })

    }
}

fn main() {
    let mut numbers:Vec<u8> = Vec::new();
    if let Ok(file) = read_file("inputs/day04inputNumbers.txt") {
        for line in file {
            for number in line.expect("That is not a valid input!").split(',') {
                numbers.push(u8::from_str_radix(number,10).unwrap());
            }
        }
    }

    let mut boards:Vec<Board> = Vec::new();
    if let Ok(file) = read_file("inputs/day04inputBoards.txt") {
        let mut board_line:usize = 0;
        let mut current_board:[[Cell;5]; 5] = [[Cell::new(0);5]; 5];
        for line in file {
            if board_line > 4 {
                board_line = 0;
                boards.push(Board{cells: current_board,hasWon: false });
                current_board = [[Cell::new(0);5]; 5];
            } else {
                let mut board_number:usize = 0;
                for number in line.expect("That is not a valid input!").split(' ') {
                    if !number.is_empty() {
                        let cell_num = u8::from_str_radix(number,10).unwrap();
                        current_board[board_line][board_number].number = cell_num;
                        board_number += 1;
                    }
                }
                board_line += 1;
            }

        }
    }

    print!("{}",boards[0]);
    for number in numbers {
        println!("---{}---",number);
        for i in 0..boards.len() {
            let board = &mut boards[i];
            if !board.hasWon && board.mark_board(number) {
                println!("{}",board);
                board.hasWon = true;
            }


        }


    }

}

fn read_file<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P : AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}