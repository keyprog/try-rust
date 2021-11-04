mod config;
mod quick_sort;

use std::env;
use std::process;
use config::Config;

fn main() {

    let args : Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {        
         println!("Failed to parse parameters: {}", err); 
         println!("Usage: sortme <input_file> <output_file> QuickSort");
         process::exit(1);
    });

    println!("Executing {:?} on file {} output to {}", &config.algo, config.input_file, config.output_file);
    match &config.algo {
        QuickSort => quick_sort::QuickSort::exec_sort(&config.input_file, &config.output_file, config.algo),
    };    
}
