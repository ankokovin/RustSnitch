mod tests;
extern crate clap;
extern crate assert_cmd;
use clap::{Arg, App, SubCommand, AppSettings};
use std::process::exit;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use aho_corasick::AhoCorasick;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct CommandMatch {
    line: String,
    line_idx: i32,
    file: String
}

pub struct InputFile {
    file: String,
    lines: io::Lines<io::BufReader<File>>
}

pub fn about() -> Result<String, String> {
    let message =   "Rust Snitch by Kokovin Aleksei \n\
                        I will hopefully write about the args here\n\
                        I'm sorry if I didn't".to_string();
    return Ok(message.to_string());
}

fn read_file<'a>(paths: Vec<String>) -> Result<Vec<InputFile>, String> {
    let mut files = vec![];
    for path in paths {
        let res = read_lines(path.clone());
        match res {
            Ok(lines) => {
                files.push(InputFile{ file: path, lines })
            },
            Err(er) =>
                return Err(format!("{:?}", er))
        }
    };
    return  Ok(files);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file_result = File::open(filename);
    match file_result {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => Err(e)
    }
}

fn search_patterns(patterns: Vec<String>,
                       lines: io::Lines<io::BufReader<File>>,
                       file_name: String) -> Result<Vec<CommandMatch>, String> {
    let ac = AhoCorasick::new(patterns);
    let mut matches = vec![];
    for line_res in lines {
        match line_res {
            Ok(line) => {
                let res = ac.find(line.clone());
                if res.is_some() {
                    matches.push(CommandMatch {
                        line: line.clone(),
                        line_idx: 0,
                        file: file_name.clone()
                    })
                }
            }
            Err(er) => {
                return Err(format!("{:?}", er))
            }
        }
    }
    return Ok(matches);
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
        let files = read_file(
            matches.value_of("files").unwrap()
                .split(",").map(|x| x.to_string()).collect());
        let patterns = vec!["TODO".to_string()];
        let mut result = vec![];
        for file in files.unwrap() {
            let file_pattern_matched = search_patterns(patterns.clone(), file.lines, file.file);
            if file_pattern_matched.is_ok() {
                result.append(&mut file_pattern_matched.unwrap());
            }
        }
        println!("{:?}", result);
    } else {

    }
}
