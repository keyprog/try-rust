use std::env;
use std::process;

fn main() {

    let args : Vec<&String> = env::args().collect();
    match Config::from_args(&args) {
        Ok(config) => { exec_sort(config.filename, config.algo); }
        Err(error) => { println!("{}", error); process.exit(1);}
    }
    println!("Hello, world!");
}

fn exec_sort(filename: &str, algo: &Algo){
    println!("Executing {:?} on file {}", algo, filename);
}

#[derive(Debug)]
struct Config {
    filename: String,
    algo: Algo,
}

impl Config {
    fn from_args(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Expected 2 arguments");
        }

        Ok(Self {
            filename: args[1].clone(),
            algo: Algo::QuickSort,        
        })
    }
}

#[derive(Debug)]
pub enum Algo {
    QuickSort
}
