mod tests;

use std::env;
use std::process::exit;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn about() -> Result<String, String> {
    let message =   "Rust Snitch by Kokovin Aleksei \n\
                        I will hopefully write about the args here\n\
                        I'm sorry if I didn't".to_string();
    return Ok(message.to_string());
}

fn read_file(paths: Vec<String>) -> Result<String, String> {
    let mut message = String::new();
    for path in paths {
        let res = read_lines(path);
        match res {
            Ok(lines) => {
                for line in lines {
                    if let Ok(ip) = line {
                        message = message + &*ip + "\n";
                    }
                }
            },
            Err(er) =>
                return Err(format!("{:?}", er))
        }
    };
    return  Ok(message);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file_result = File::open(filename);
    match file_result {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => Err(e)
    }

}

fn run(args: Vec<String>) -> Result<String, String> {
    if args.len() == 1 {
        println!("I expected some args!");
        return about();
    }
    if args[1] == "-f" {
        if args.len() == 2 {
            return Err("Expected path to file".to_string())
        }
        return read_file(args[2..].to_owned());
    }
    return Err("Could not parse args!".to_string());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match run(args) {
        Ok(message) => {
            println!("{}", message);
        },
        Err(error_message) => {
            println!("{}", error_message);
            exit(1);
        }
    }
}
