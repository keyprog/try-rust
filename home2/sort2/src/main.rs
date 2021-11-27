use rand::Rng;
use std::mem;

fn main() {
    let mut rnd = rand::thread_rng();
    let var1 : u32 = rnd.gen();

    println!("Random: {}", var1);
    println!("Random: {}", rnd.gen::<u32>());
    println!("Random: {}", rnd.gen_range(0..10));


    let input: [i32; 100] = [0; 100];
    test(&input);
    println!("{:?}", input);
}

fn test(input: &[i32]) {
    //println!("{:?}", input[0..3]);
    println!("{}", mem::size_of_val(&input));
}
