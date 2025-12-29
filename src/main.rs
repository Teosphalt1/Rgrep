use std::env;
use std::process;

mod check_arguments;
mod read_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if check_arguments::check_args(&args) == false {
        process::exit(1);
    }
    else {
        match read_file::read_file(&args[2], &args[1]) {
            Ok(lines) => {
                if lines.is_empty() {
                    println!("No matches found.");
                } else {
                    for line in lines {
                        println!("{} | Has been found line {}", line.line_text, line.line_number);
                    }
                }
            },
            Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
            }
        }
    }
}

