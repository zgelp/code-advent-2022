use std::fs::File;
use std::path::Path;
use std::io::Read;


fn char_to_score(c: char) -> u32 {
    let ascii_code = c as u32;
    // 65-90 = A-Z
    if (65..=90).contains(&ascii_code) {
        ascii_code - 65 + 27
    } else {
    // 97-122 = a-z
    ascii_code - 97 + 1
    }
}

fn first_part(lines: &Vec<&str>) -> () {
    let mut all_line_scores: Vec<u32> = Vec::new();
    for line in lines {
        let line_middle: usize = line.chars().count() / 2;
        let first: &str = &line[..line_middle];
        let second: &str = &line[line_middle..];
        let mut score: u32 = 0;
        for i in first.chars() {
            if second.contains(i) {
                score += char_to_score(i);
                break
            }
        }
        all_line_scores.push(score)

    }
    let sum: u32 = all_line_scores.iter().sum();
    println!("First part sum {:?}", sum);
    
}

fn second_part(lines: &Vec<&str>) -> () {
    let mut i = 0;
    let mut score = 0;
    while i < lines.len() {
        let first_backpack = lines[i];
        let second_backpack = lines[i + 1];
        let third_backpack = lines[i + 2];
        
        i += 3;

        for item in first_backpack.chars() {
            if second_backpack.contains(item) && third_backpack.contains(item) {
                score += char_to_score(item);
                break
            }
        }
    }
    println!("Score part 2: {:?}", score);
}


fn main() {
    let path = Path::new("rucsak.txt");

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
    first_part(&lines);

    second_part(&lines);
}
