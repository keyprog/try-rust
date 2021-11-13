#[derive(Debug)]
pub struct Config {
    pub input_file: String,
    pub output_file: String,
    pub algo: Algo,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Self, &str> {
        if args.len() < 4 {
            return Err("Expected 2 arguments");
        }

        Ok(Self {
            input_file: args[1].clone(),
            output_file: args[2].clone(),
            algo: args[3].parse().unwrap(),
        })
    }
}

#[derive(Debug)]
pub enum Algo {
    QuickSort, MergeSort
}

impl std::str::FromStr for Algo {
    type Err = String;

    fn from_str(s: &str)-> Result<Self, Self::Err> {
        return match s {
            "QuickSort" => Ok(Self::QuickSort),
            "MergeSort" => Ok(Self::MergeSort),
            _ => Err(format!("Unexpected algo")),
        };
    }
}
