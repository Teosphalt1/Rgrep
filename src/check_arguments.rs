use std::fs;

pub fn check_args(arguments: &Vec<String>) -> bool {
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