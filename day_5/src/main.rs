use std::fs::File;
use std::path::Path;
use std::io::Read;
use csv::ReaderBuilder;
use std::collections::HashMap;
use itertools::Itertools;
use csv::Reader;


fn prepare_data(mut reader: Reader<&[u8]>) -> HashMap<usize, Vec<String>> {
    let mut csv_data: HashMap<usize, Vec<String>> = HashMap::with_capacity(10);
    // Parse csv and create hashmap
    for result in reader.records() {
        let line = result.unwrap();
        for i in 0..9 {
            let one_char = line[i].to_owned();
            if csv_data.contains_key(&i) {
                let v = csv_data.get_mut(&i).unwrap();
                v.push(one_char);
            } else {
                csv_data.insert(i, vec![one_char]); 
            }
        }
    }

    // Sort vector values in hashmap and remove empty strings
    let mut prepared_data: HashMap<usize, Vec<String>> = HashMap::new();
    for (i, vec) in csv_data.iter() {
        let reversed_vec: Vec<String> = vec.iter().rev().filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        }).collect();
        // Increment Hashmap id to get indexes as in example table
        prepared_data.insert(*i + 1, reversed_vec);
    }
    prepared_data
}

fn prepare_instructions(instructions: Vec<&str>) -> Vec<(usize,usize,usize)>{
    instructions.into_iter()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .filter(|words| words[1].parse::<usize>().is_ok() && words[3].parse::<usize>().is_ok() && words[5].parse::<usize>().is_ok())
        .map(|words| (words[1].parse::<usize>().unwrap(), words[3].parse::<usize>().unwrap(), words[5].parse::<usize>().unwrap())).collect()
}

fn arrange_supply_first_part(instructions: Vec<(usize, usize, usize)>, mut supply: HashMap<usize, Vec<String>>)  {
    for instruction in instructions {
        // Take first vector from hashmap (based on instruction id)
        let first_vec = supply.get_mut(&instruction.1).unwrap();
        // Take items from that vector and reverse
        let mut vec_elements: Vec<String> = first_vec.drain(first_vec.len() - instruction.0..).collect();
        // Only difference between first / second part
        vec_elements.reverse();
        // Update second vector based on ID
        let second_vec = supply.get_mut(&instruction.2).unwrap();
        second_vec.extend(vec_elements);
    }

    println!("Solution First Part: ");
    for (k, v) in supply.iter().sorted_by_key(|x| x.0) {
        println!("{:?} => {:?}", k, v);
    }
}
fn arrange_supply_second_part(instructions: Vec<(usize, usize, usize)>, mut supply: HashMap<usize, Vec<String>>)  {
    for instruction in instructions {
        // Take first vector from hashmap (based on instruction id)
        let first_vec = supply.get_mut(&instruction.1).unwrap();
        // Take items from that vector
        let vec_elements: Vec<String> = first_vec.drain(first_vec.len() - instruction.0..).collect();
        // Update second vector based on ID
        let second_vec = supply.get_mut(&instruction.2).unwrap();
        second_vec.extend(vec_elements);
    }

    println!("Solution Second Part: ");
    for (k, v) in supply.iter().sorted_by_key(|x| x.0) {
        println!("{:?} => {:?}", k, v);
    }
}

fn main() {
    let path = Path::new("supply.txt");

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
    let data: Vec<&str> = lines[0..8].to_vec();
    let instructions: Vec<&str> = lines[10..].to_vec();
    let instructions2: Vec<&str> = lines[10..].to_vec();
    let mut csv_data: Vec<String> = Vec::new();

    // LMAO 💀 💀 💀
    for line in data {
        let replace1 = line.replace("            ", ",,,");
        let replace2 = replace1.replace("         ", ",,,");
        let replace3 = replace2.replace("     ", ",,");
        let replace4 = replace3.replace("    ", ",");
        let replace5 = replace4.replace(" ", ",");
        csv_data.push(replace5);
    }

    let string_from_vec = csv_data.join("\n");
    let rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(string_from_vec.as_bytes());
    let rdr2 = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(string_from_vec.as_bytes());

    let supply_data: HashMap<usize, Vec<String>> = prepare_data(rdr);
    let supply_instructions = prepare_instructions(instructions);
    let supply_instructions2 = prepare_instructions(instructions2);
    let supply_data2: HashMap<usize, Vec<String>> = prepare_data(rdr2);

    arrange_supply_first_part(supply_instructions, supply_data);
    arrange_supply_second_part(supply_instructions2, supply_data2);
}