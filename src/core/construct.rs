use super::*; 

impl <T: Ord> BiHeap<T> {
    pub fn new() -> Self {
        let bivec = BiVec::new(); 
        BiHeap(Rc::new(RefCell::new(bivec)))
    }
    pub fn with_capacity(capacity: usize) -> Self {
        let bivec = BiVec::with_capacity(capacity); 
        BiHeap(Rc::new(RefCell::new(bivec)))
    } 
}