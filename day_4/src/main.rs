use std::fs::File;
use std::path::Path;
use std::io::Read;


fn check_arrays(arr: Vec<Vec<u32>>) -> bool {
    let first_contains = arr[0].iter().all(|x| arr[1].contains(x));
    let second_contains = arr[1].iter().all(|x| arr[0].contains(x));

    first_contains || second_contains
}

fn parse_data(lines: Vec<&str>) {
    let mut score: u32 = 0;
    for line in lines {
        let splited_line = line.split(',').collect::<Vec<&str>>();
        let mut arr : Vec<Vec<u32>>= Vec::new();
        for one_part in splited_line {
            let splitted_into_int = one_part.split('-').collect::<Vec<&str>>();
            let first_int: u32 = splitted_into_int[0].parse().unwrap();
            let second_int: u32 = splitted_into_int[1].parse().unwrap();
            let array_with_range: Vec<u32> = (first_int..=second_int).collect();
            arr.push(array_with_range);            
        }
        ;
        score += u32::from(check_arrays(arr));
    }
    println!("Frist part score: {:?}", score);
}

fn main() {
    let path = Path::new("sections.txt");

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
    parse_data(lines);
}
