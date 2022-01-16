use modules::my_mods;

fn main() {
    println!("Hello, world!");
    let test = my_mods::my_mod1::Test{ test : 5};
    println!("{:?}", test);
}