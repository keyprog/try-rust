use std::io;
use std::io::Write;

fn main() {
    println!("Fibonacci number. v.0.1");
    let mut input = String::new();    

    loop {
        print!("Input a number: ");
        io::stdout().flush().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input : i32 = match input.trim().parse() {
            Ok(num) if num > 10 => num,            
            Err(_) => {
                println!("A number was expected");
                continue;
            },
            _ => { continue;}
        };
        println!("Result: {}", fib(input));

    };
}

fn fib(num: i32) -> i32 {
    match num {
        0 => return 0,
        1 => return 1,
        2 => return 1,
        _ => return fib(num - 1)+ fib(num - 2),
    }
}
