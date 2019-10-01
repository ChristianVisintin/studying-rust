mod grep;
/**
 * Author: Christian Visintin
 * Description: minigrep takes as argument the filename to search in and the string to look for
 */
//Project includes
mod reader;

extern crate getopts;
use getopts::Options;

use std::env;

fn usage(program: &str, options: Options) {
    let brief = format!("Usage: {} PATTERN [options]", program);
    println!("{}", options.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    //Get options
    let mut opts = Options::new();
    opts.optopt("f", "file", "Set file to search in", "");
    opts.optopt("i", "in", "Set string to search in", "");
    opts.optflag("h", "help", "show this page");
    //Match options
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            usage(&program, opts);
            std::process::exit(1);
        }
    };
    if matches.opt_present("h") {
        //Show usage
        usage(&program, opts);
        return;
    }
    let input_file: Option<String> = matches.opt_str("f");
    let mut input_string = matches.opt_str("i");
    let pattern = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("Missing pattern");
        usage(&program, opts);
        std::process::exit(1);
    };
    //If input file
    match input_file {
        Some(file) => {
            input_string = match reader::read_file(&file) {
                Some(cnt) => Some(cnt),
                None => {
                    println!("Could not read file '{}'", file);
                    std::process::exit(1);
                }
            }
        }
        None => (),
    };

    //Match pattern
    let haystack: String;
    match input_string {
        Some(s) => haystack = s,
        None => {
            println!("No string to search in was provided");
            usage(&program, opts);
            std::process::exit(1);
        }
    };
    let result = grep::match_pattern(&haystack, &pattern);
    //Print results
    match result {
        Ok(matches) => {
            for match_str in matches {
                println!("{}", match_str);
            }
        }
        Err(err) => {
            println!("Could not match '{}' in '{}': {}", pattern, haystack, err);
            std::process::exit(1);
        }
    };
}
