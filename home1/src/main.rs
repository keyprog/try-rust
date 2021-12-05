
fn main() {
    let a: bool = false;
    
    println!("Hello, world! {}", a);

    let mut heap_vector : Vec<i32> = Vec::new();
    for i in 0..10 {
        heap_vector.push(i);
    }

    let aa = a;
    println!("{}", a);
    println!("{}", aa);

    //let hv = heap_vector;
    //println!("{}",heap_vector.len());
    let f : f32 = 0.0;
    let mut fb :  Box<f32> = Box::new(1.23);

    println!("{0} {1}", f, fb);
    fb = tst(fb);
    println!("{}",fb);

    let s : &str = "test";
    let s2 : String = String::from("ttest");

    println!("{} {}", s, s2);
}

fn tst(obj : Box<f32>) -> Box<f32>{
    println!("{}", obj);
    return obj;
}
