use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // argument handling
    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("search for patterns")
        .author("Frank Schoenmann")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("The input file to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        //let contains_substring = re.find(line);
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}:{}", line_num + 1, line),
            None => (),
        }
    }
}
