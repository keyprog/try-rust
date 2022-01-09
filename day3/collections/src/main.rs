use std::collections::HashMap;

fn main() {
    
    let mut my_vec = vec![5, 6, 7];

    let my_str = &mut my_vec[0];
    *my_str += 10;

    println!("{}", my_str);
    println!("{:?}", my_vec);

    my_vec.get(1);

    let mut my_hash = HashMap::new();

    my_hash.insert(12, String::from("bla"));

    println!("{:?}", my_hash);

    match my_hash.get(&12) {
        Some(val) => println!("{}", val) ,
        None =>  println!("Nothing") 
    };

}
