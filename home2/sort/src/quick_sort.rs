use super::*;

pub struct QuickSort {

}

impl QuickSort {    
    pub fn exec_sort(input_file: &str, output_file: &str, algo: config::Algo){
        println!("Executing {:?} on {} to {}", algo, input_file, output_file);
    }
}