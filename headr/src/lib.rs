use std::error::Error;
use std::fmt::format;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("szhao")
        .about("about")
        .arg(
            Arg::with_name("lines")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .help("number of lines")
            .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .value_name("BYTES")
            .takes_value(true)
            .conflicts_with("lines")
            .help("number of bytes")
        )
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("input file(s)")
            .multiple(true)
            // .default_value("-")
        )
        .get_matches();
    let lines = matches.value_of("lines").map(parse_positive_int).transpose().map_err(|e| format!("illegal line count-- {}", e))?;
    
    let bytes = matches.value_of("bytes").map(parse_positive_int).transpose().map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config { 
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes, })
}

pub fn run(){

}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}