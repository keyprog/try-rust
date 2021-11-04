use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = filereader::Config {
        filename : args[1].clone(),
    };

    println!("{:?}", config);

    if args.len() < 2 {
        println!("Usage: file_reader <file_name>");
        process::exit(1);
    }

    let filename = &args[1];

    println!("Trying to open file '{}'", filename);

    let content = match fs::read_to_string(filename)  {
        Ok(str) => str,
        Err(err) => { println!("{}", err); return; }
    };

    let file = File::open(filename).unwrap();
    
    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            println!("Line: {}", l);
        }
    }
    

    println!("{}", content);
}
