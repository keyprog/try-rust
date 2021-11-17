use rand::prelude;
use std::mem;

fn main() {
    let input: [i32; 100] = [0; 100];
    test(&input);
    println!("{:?}", input);
}

fn test(input: &[i32]) {
    println!("{:?}", input[0..3]);
    println!("{}", mem::size_of_val(&input));
}
