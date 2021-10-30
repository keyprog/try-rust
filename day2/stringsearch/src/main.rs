fn main() {
    
    let index = find_str("hello world", "ld").unwrap();
    println!("Found at index {}", index);
}

fn find_str(str: &str, search: &str ) -> Option<usize> {
    
    if str.len() < search.len(){
        return None;
    }

    if str.len() == 0 {
        return None;
    }

    for i in 0..str.len() - search.len() + 1 {
        if &str[i..i+search.len()] == search{
           return Some(i);
        }        
    }

    return None;
}
