use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if check_args(args) == false {
        println!("You failed!");
    }
    else {
        println!("You passed!");
    }
}

fn check_args(arguments: Vec<String>) -> bool {
    if arguments.len() < 3 {
        println!("Two arguments are expected.");
        false
    }
    else if arguments.len() > 3 {
        println!("Only two arguments are expected.");
        false
    }
    else {
        if check_file_exists(&arguments[2]) {
            true
        } else {
            println!("No such file or directory.");
            false
        }
    }
}

fn check_file_exists(path: &String) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false
    }
}
