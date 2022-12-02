use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;

#[derive(Debug)]
struct Elf {
    pub id: usize,
    pub calories_sum: u32,
    pub calories: Vec<u32>
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn prepare_calories(all_calories: io::Lines<io::BufReader<File>>) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let mut prepared: Vec<Vec<u32>> = Vec::new();
    prepared.push(Vec::new());
    for line in all_calories {
        if let Ok(ip) = line {
            if ip.is_empty() {
                prepared.push(Vec::new());
            }
            else {
                let calories: u32 = ip.trim().parse().unwrap();
                prepared.last_mut().unwrap().push(calories);
            }
        }
    }
    Ok(prepared)
}


fn create_elfs(all_calories: Vec<Vec<u32>>) -> Result<Vec<Elf>, Box<dyn Error>> {
    let mut elfs: Vec<Elf> = Vec::new();
    for elf_calories in &all_calories {
        let elf_index: usize = all_calories.iter().position(|x| x == elf_calories).unwrap();
        let calories_sum: u32 = elf_calories.iter().sum();
        elfs.push(Elf{id: elf_index, calories_sum: calories_sum, calories: elf_calories.to_vec() })
    }
    Ok(elfs)
}


fn main() {
    if let Ok(lines) = read_lines("./calories.txt") {
        let prepared_calories: Vec<Vec<u32>> = prepare_calories(lines).unwrap();
        let mut elfs: Vec<Elf> = create_elfs(prepared_calories).unwrap();
        elfs.sort_by(|a, b| b.calories_sum.cmp(&a.calories_sum));
        let top_three_sum: u32 = elfs[0..=2].iter().map(|s| s.calories_sum).sum();

        println!("Top calories elf: {:?}", elfs[0]);
        println!("Top three elves calories sum: {:?}", top_three_sum);
    }
}

