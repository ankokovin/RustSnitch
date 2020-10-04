mod tests;

use std::env;
use std::process::exit;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ResultType {
    OK,Fail
}

#[derive(Debug, Clone)]
pub struct ProgramResult {
    type_result: ResultType,
    message: Option<String>,
}

pub fn about() -> Box<ProgramResult> {
    let message =   "Rust Snitch by Kokovin Aleksei \n\
                        I will hopefully write about the args here\n\
                        I'm sorry if I didn't".to_string();
    let type_result = ResultType::OK;
    return Box::new(ProgramResult { type_result, message: Some(message) });
}

fn read_file(paths: Vec<String>) -> Box<ProgramResult> {
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
            }
            Err(er) => {
                return Box::new(ProgramResult { type_result: ResultType::Fail, message: Option::from(format!("{:?}", er)) });
            }
        }
    }
    return  Box::new(ProgramResult { type_result: ResultType::OK, message: Option::from(message) });
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file_result = File::open(filename);
    match file_result {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => Err(e)
    }

}

fn act_result(result: Box<ProgramResult>) {
    let default_message = "Unknown error".to_string();
    println!("{}", result.message.unwrap_or(default_message));
    match result.type_result {
        ResultType::OK => exit(0),
        ResultType::Fail => exit(1),
    }
}

fn run(args: Vec<String>) -> Box<ProgramResult> {
    let result: Box<ProgramResult> = Box::new(ProgramResult { type_result: ResultType::Fail, message: None });
    if args.len() == 1 {
        println!("I expected some args!");
        return about();
    }
    if args[1] == "-f" {
        if args.len() == 2 {
            return Box::new(ProgramResult{type_result: ResultType::Fail, message: Option::from("Expected path to file".to_string()) })
        }
        return read_file(args[2..].to_owned());
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = run(args);
    act_result(result);
}
