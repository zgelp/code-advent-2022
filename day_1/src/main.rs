use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct Elf {
    pub id: u16,
    pub calories: u32
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./calories.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                if ip.is_empty() {
                    println!("IS EMPTY")
                }
                else {
                    let calories: u32 = ip.trim().parse().unwrap();
                    Elf::default()
                    println!("{}", calories)
                }
            }
        }
    }
}

