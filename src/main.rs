mod tests;
extern crate clap;
extern crate assert_cmd;
use clap::{Arg, App, SubCommand, AppSettings};
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

fn main() {
    let matches = App::new("RustSnitch")
        .version("0.1")
        .author("Kokovin Aleksei <rycarok@gmail.com>")
        .about(about().unwrap().as_str())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("files")
            .short("f")
            .value_name("files")
            .help("Select files to parse. Separate by commas.")
            .multiple(true)
            .takes_value(true)).get_matches();
    if matches.value_of("files").is_some() {
        let result = read_file(
            matches.value_of("files").unwrap()
                .split(",").map(|x| x.to_string()).collect());
        match result {
            Ok(message) => {
                println!("{}", message)
            },
            Err(error_message) => {
                eprintln!("{}", error_message);
                exit(1)
            }
        }
    } else {

    }
}
