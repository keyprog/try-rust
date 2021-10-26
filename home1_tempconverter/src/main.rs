use std::io;
use std::io::prelude::*;

fn main() {
    println!("Temperature converter v.1.0");

    loop {
        print!("Enter termperature (e.g. 20C or 32F): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input");

        let (temp_type, temp_value) = match parse_temp(input.trim()) {
            Some(val) => val,
            _ => {
                println!("Unable to parse input");
                continue;
            }
        };

        let (converted_type_name, converted_value) = match convert_temp(temp_type, temp_value) {
            Ok(val) => val,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!("You provided: {temp} {type}", temp=temp_value, type=get_temp_name(temp_type));
        println!("Converted: {temp} {type}", temp=converted_value, type=get_temp_name(converted_type_name));
    }
}

fn parse_temp(input: &str) -> Option<(char, f32)> {
    if input.len() < 2 {
        return None;
    }

    let temp_type = input.chars().last().unwrap().to_ascii_uppercase();
    return match input[..input.len() - 1].trim().parse() {
        Ok(temp_value) => Some((temp_type, temp_value)),
        _ => None,
    };
}

fn convert_temp(temp_type: char, temp_value: f32) -> Result<(char, f32), &'static str> {
    return match temp_type {
        'C' => Ok(('F', (temp_value * 9.0 / 5.0) + 32.0)),
        'F' => Ok(('C', (temp_value - 32.0) * 5.0 / 9.0)),
        _ => Err("Unsupported type. Only C and F supported"),
    };
}

fn get_temp_name(temp_type: char) -> &'static str {
    return match temp_type {
        'C' => "Celcius",
        'F' => "Fahrenheit",
        _ => {
            panic!("Unsupported type. Only C and F supported");
        }
    };
}
