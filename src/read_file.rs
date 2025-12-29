use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file(path: &String, substring_to_find: &String) -> Result<Vec<String>, std::io::Error>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains(substring_to_find) {
            result.push(line);
        }
    }
    Ok(result)
}