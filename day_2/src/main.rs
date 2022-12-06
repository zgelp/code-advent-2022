use std::fs::File;
use std::path::Path;
use std::io::Read;


#[derive(Debug, PartialEq, Eq)]
enum PlayedHand {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(PartialEq, Eq)]
enum EndResult {
    Win,
    Lose,
    Draw 
}

struct Strat {
    player1: PlayedHand,
    player2: PlayedHand
}

struct FullStrat {
    player1: PlayedHand,
    player2_outcome: EndResult
}

fn first_strat(strats: Vec<Strat>) {
    let mut score = 0;
    for strat in strats {
        // Lose.
        if (strat.player1 == PlayedHand::Paper && strat.player2 == PlayedHand::Rock) ||
            (strat.player1 == PlayedHand::Scissors && strat.player2 == PlayedHand::Paper) ||
            (strat.player1 == PlayedHand::Rock && strat.player2 == PlayedHand::Scissors) {
            score += strat.player2 as i32;
            continue;
        }
        // Draw.
        if strat.player1 == strat.player2 {
            score += 3 + strat.player2 as i32;
            continue;
        }
        // Win
        score += 6 + strat.player2 as i32
    }
    println!("First strat score: {}", score);
}

fn full_strat(strats: Vec<FullStrat>) {
    let mut score = 0;
    for strat in strats {
        // Outcome = Lose
        if strat.player2_outcome == EndResult::Lose {
            let player2: PlayedHand = match strat.player1 {
                PlayedHand::Rock => PlayedHand::Scissors,
                PlayedHand::Scissors => PlayedHand::Paper,
                PlayedHand::Paper => PlayedHand::Rock
            };
            score += player2 as u32;
            continue;
        }
        // Outcome = Draw
        if strat.player2_outcome == EndResult::Draw {
            score += 3 + strat.player1 as u32;
            continue;
        }
        // Outcome = Win
        if strat.player2_outcome == EndResult::Win {
            let player2: PlayedHand = match strat.player1 {
                PlayedHand::Scissors => PlayedHand::Rock,
                PlayedHand::Rock => PlayedHand::Paper,
                PlayedHand::Paper => PlayedHand::Scissors
            };
            score += 6 + player2 as u32;
        }
    }
    println!("Full strat score: {}", score);
}

fn prepare_first_strat_data(lines: Vec<&str>) -> Vec<Strat> {
    let first_strat_data: Vec<Strat> = lines.iter().map(|&val| {
        let line_split: Vec<&str> = val.split(' ').collect();
        let player1: PlayedHand = match line_split[0] {
            "A" => PlayedHand::Rock,
            "B" => PlayedHand::Paper,
            "C" => PlayedHand::Scissors,
            &_ => todo!(),
        };
        let player2: PlayedHand = match line_split[1] {
            "X" => PlayedHand::Rock,
            "Y" => PlayedHand::Paper,
            "Z" => PlayedHand::Scissors,
            &_ => todo!(),
        };
        Strat {
            player1,
            player2
        }
    }).collect();
    first_strat_data
}

fn prepare_full_strat_data(lines: Vec<&str>) -> Vec<FullStrat>{
    let full_strat_data: Vec<FullStrat> = lines.iter().map(|&val| {
        let line_split: Vec<&str> = val.split(' ').collect();
        let player1: PlayedHand = match line_split[0] {
            "A" => PlayedHand::Rock,
            "B" => PlayedHand::Paper,
            "C" => PlayedHand::Scissors,
            &_ => todo!(),
        };
        let player2_outcome: EndResult = match line_split[1] {
            "X" => EndResult::Lose,
            "Y" => EndResult::Draw,
            "Z" => EndResult::Win,
            &_ => todo!(),
        };
        FullStrat{
            player1,
            player2_outcome
        }
    }).collect();
    full_strat_data
}

fn main() {
    let path = Path::new("strategy.txt");

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    let contents: &str = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => &s,
    };

    let lines: Vec<&str> = contents.split('\n').collect();
    first_strat(prepare_first_strat_data(lines));

    let full_lines: Vec<&str> = contents.split('\n').collect();
    full_strat(prepare_full_strat_data(full_lines));
}
