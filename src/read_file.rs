use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct LineInfo {
    pub line_text: String,
    pub line_number: i32,
}

pub fn read_file(path: &String, substring_to_find: &String) -> Result<Vec<LineInfo>, std::io::Error>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result: Vec<LineInfo> = Vec::new();
    let mut counter = 1;
    for line in reader.lines() {
        let line = line?;

        if line.contains(substring_to_find) {
            result.push(LineInfo{
                line_text: line,
                line_number: counter
            });
        }
        counter += 1;
    }
    Ok(result)
}