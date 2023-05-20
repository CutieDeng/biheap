use super::*; 

impl <'a, T: Ord> ViewMut<'a, T> {
    pub fn as_ref(&self) -> &T {
        unsafe { &self.node.try_borrow_unguarded().unwrap().value } 
    }
    pub fn set_value(&mut self, mut value: T) -> T {
        std::mem::swap(&mut value, &mut self.node.borrow_mut().value); 
        let bor = self.node.borrow(); 
        let min_index = bor.min_index; 
        let max_index = bor.max_index; 
        drop(bor); 
        self.bi_heap.bubble_down::<true>(min_index);
        self.bi_heap.bubble_up::<true>(min_index); 
        self.bi_heap.bubble_down::<false>(max_index); 
        self.bi_heap.bubble_up::<false>(max_index); 
        value 
    } 
    pub fn pop_value(self) -> T {
        let bor = self.node.borrow(); 
        let min_index = bor.min_index; 
        let max_index = bor.max_index; 
        drop(bor); 
        let mut bivec = self.bi_heap.0.borrow_mut(); 
        bivec.swap_remove(min_index, max_index); 
        let [slice1, slice2]  = bivec.views_mut(); 
        slice1[min_index].borrow_mut().min_index = min_index; 
        slice2[max_index].borrow_mut().max_index = max_index; 
        drop(bivec); 
        self.bi_heap.bubble_down::<true>(min_index);
        self.bi_heap.bubble_up::<true>(min_index); 
        self.bi_heap.bubble_down::<false>(max_index); 
        self.bi_heap.bubble_up::<false>(max_index); 
        let node = self.node;
        let node = Rc::try_unwrap(node).ok().unwrap(); 
        let node = node.into_inner(); 
        node.value 
    }
}