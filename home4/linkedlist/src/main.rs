fn main() {
    println!("Hello, world!");
    let mut list = LinkedList::new();
    println!("{:?}", list);
    list = list.add(10).add(11).add(20);
    
    println!("{}", list.len());
    println!("{:?}", list);
    if let Some((tail, val)) = list.get() {
        list = tail;
        println!("Extracted {}", val);
        println!("{:?}", list);
    }
}

#[derive(Debug)]
enum LinkedList {
    Node(Box<LinkedList>, u32),
    End,
}

impl LinkedList {
    fn new() -> Self {
        return Self::End;
    }

    fn add(self, val:u32) -> Self {
        Self::Node(Box::new(self), val)
    }

    fn get(self) -> Option<(Self, u32)> {
        match self {
            LinkedList::Node(tail, val) => Some((*tail, val)),
            LinkedList::End => None            
        }
    }

    fn len(&self) -> usize {
        match &*self {
            Self::Node(tail, _) => 1+tail.len(),
            Self::End => 0            
        }
    }
}