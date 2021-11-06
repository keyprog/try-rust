mod config;
mod quick_sort;

use std::env;
use std::process;
use config::Config;
use std::fs::File;
use std::io::{self};
use std::io::BufRead;

fn main() {

    let args : Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {        
         println!("Failed to parse parameters: {}", err); 
         println!("Usage: sortme <input_file> <output_file> QuickSort");
         process::exit(1);
    });

    println!("Executing {:?} on file {} output to {}", &config.algo, config.input_file, config.output_file);
    

    let mut input : Vec<String> = read_lines(&config.input_file).unwrap_or_else(|err| {
        println!("Failed to read input file: {}", err);
        process::exit(1);
    });

    match &config.algo {
        config::Algo::QuickSort => quick_sort::QuickSort::exec_sort(&mut input),
    };
}

fn read_lines(file_name: &str) -> Result<Vec<String>, &str>{
    let file = File::open(file_name).expect("Failed to open file");
    let lines = io::BufReader::new(file);
    let lines : Vec<String> = lines.lines().map(|l|l.expect("unable to read line")).collect();
    return Ok(lines);
}
