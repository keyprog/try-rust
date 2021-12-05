use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut my_arr : [i32; 4] = [1, 2, 3, 4];
    my_arr[1] = 44;
    println!("Command line {:?}", args);

    for e in &my_arr {
        println!("{}", e);
    }

    println!("Command line {:?}", my_arr)

}