#[derive(Debug)]
pub struct Config {
    pub filename : String,
}

impl Config {
    pub fn new(fname: &String) -> Config {
        Self {
            filename : fname.clone(),
        }
    }
}