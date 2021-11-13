pub struct QuickSort {

}

impl QuickSort {    
    pub fn exec_sort(mut input: &mut Vec<String>){
        //for line in input.clone() {
            //println!("{}", line);
        //}

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
        
        let pi = Self::partition(&mut input, start, end-1);

        Self::quick_sort(&mut input, start, pi);
        Self::quick_sort(&mut input, pi+1, end);
    }

    fn partition(input: &mut Vec<String>, start: usize, end: usize) -> usize {
        let piv = input[end].clone();
        let mut low = start;
        for i in start..end {
            if input[i] < piv {                
                input.swap(low, i);                
                low = low + 1;
            }

        }
        input.swap(low, end);
        return low;
    }


}