pub struct QuickSort {

}

impl QuickSort {    
    pub fn exec_sort(mut input: &mut Vec<String>){
        for line in input.clone() {
            println!("{}", line);
        }

        let end = input.len();
        Self::quick_sort(&mut input, 0, end);
        for line in input.clone() {
            println!("{}", line);
        }
    }

    fn quick_sort(mut input: &mut Vec<String>, start: usize, end: usize) {
        if start == end {
            return;
        }
        
        let pi = Self::partition(&mut input, start, end);

        Self::quick_sort(&mut input, start, pi);
        Self::quick_sort(&mut input, pi+1, end);
    }

    fn partition(input: &mut Vec<String>, start: usize, end: usize) -> usize {
        let piv = input[];
        for i in start+1..end {
            if input[pi] > input[i] {
                input.swap(pi, i);                
                pi = i;
            }

        }
        return pi;
    }


}