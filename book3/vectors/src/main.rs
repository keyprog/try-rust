fn main() {
    let vec2 : Vec<u32> = Vec::with_capacity(23);
    let mut my_vec : Vec<u32> = (1..100).collect();
    my_vec.reverse();
    println!("{:?}", my_vec);
    my_vec.reverse();
}
