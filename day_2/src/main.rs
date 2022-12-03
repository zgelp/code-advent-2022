use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;


struct PlayedHand {
    score: u8
}

enum PlayResult {
    Win,
    Lose,
    Tie
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn remove_whitespace(x : String) -> String{
    x.replace(" ", "")
  }

fn main() {
    if let Ok(lines) = read_lines("./strategy.txt") {
        for line in lines {
            println!("{:?}", remove_whitespace(line.unwrap()));
        }
    }
}
