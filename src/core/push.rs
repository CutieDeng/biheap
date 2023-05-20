use super::*; 

impl <T: Ord> BiHeap<T> {
    pub fn push(&mut self, value: T) -> Handle<T> {
        let mut borrow = self.0.borrow_mut(); 
        let len = borrow.len(); 
        let value_rc = Rc::new(RefCell::new(
            Node {
                value,
                min_index: len, 
                max_index: len,
            }
        )); 
        let value_rc2 = value_rc.clone(); 
        let node_ref = Rc::downgrade(&value_rc); 
        borrow.push(value_rc, value_rc2); 
        drop(borrow); 
        self.bubble_up::<true>(len);
        self.bubble_up::<false>(len); 
        let handle = Handle {
            node_ref, 
            heap_ref: Rc::downgrade(&self.0), 
        }; 
        handle
    }
}