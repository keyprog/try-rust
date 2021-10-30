

fn main() {
    
    let mut s : String = String::from("test");
    let s2 = &s;
    
    s.push_str("test");
    
    println!("{} {}", s, s2);


}
