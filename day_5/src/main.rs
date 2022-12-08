use std::fs::File;
use std::path::Path;
use std::io::Read;
use csv::ReaderBuilder;
use std::error::Error;
use std::collections::HashMap;


// zamenji presledke z , ez csv gang rise up

fn main() -> Result<(), Box<dyn Error>>{
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
    let mut csv_data: Vec<String> = Vec::new();

    // LMAO 😂😂😂 💀 💀 💀 
    for line in data {
        let replace1 = line.replace("            ",",,,");
        let replace2 = replace1.replace("         ",",,,");
        let replace3 = replace2.replace("     ", ",,");
        let replace4 = replace3.replace("    ",",");
        let replace5 = replace4.replace(" ", ",");
        csv_data.push(replace5);
    }

    let string_from_vec = csv_data.join("\n");
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(string_from_vec.as_bytes());

    let mut csv_data: HashMap<usize, Vec<String>> = HashMap::with_capacity(10);
    
    for result in rdr.records() {
        let record = result.unwrap();
        for i in 0..9 {
            let one_char = record[i].to_owned();
            if csv_data.contains_key(&i) {
                let v = csv_data.get_mut(&i).unwrap();
                v.push(one_char);
            } else {
                csv_data.insert(i, vec![one_char]); }
        }
    }

    let mut prepared_data: HashMap<usize, Vec<String>> = HashMap::new();
    for (i, vec) in csv_data.iter() {
        let reversed_vec: Vec<String> = vec.iter().rev().filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        }).collect();
        prepared_data.insert(*i, reversed_vec);
    }

    println!("{:?}", prepared_data);
    Ok(())
}
