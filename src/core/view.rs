use super::*; 

impl <'a, T: Ord> ViewMut<'a, T> {
    pub fn peek(&self) -> &T {
        unsafe { &self.node.try_borrow_unguarded().unwrap().value } 
    }
    pub fn set(&mut self, mut value: T) -> T {
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
    pub fn pop(self) -> T {
        let bor = self.node.borrow(); 
        let min_index = bor.min_index; 
        let max_index = bor.max_index; 
        drop(bor); 
        let mut bivec = self.bi_heap.0.borrow_mut(); 
        bivec.swap_remove(min_index, max_index); 
        let [slice1, slice2]  = bivec.views_mut(); 
        let mut min_exist = false; 
        let mut max_exist = false; 
        slice1.get_mut(min_index).map(|f| { f.borrow_mut().min_index = min_index ; min_exist = true; } ); 
        slice2.get_mut(max_index).map(|f| { f.borrow_mut().max_index = max_index ; max_exist = true; } ); 
        drop(bivec); 
        if min_exist {
            self.bi_heap.bubble_down::<true>(min_index);
            self.bi_heap.bubble_up::<true>(min_index); 
        }
        if max_exist {
            self.bi_heap.bubble_down::<false>(max_index); 
            self.bi_heap.bubble_up::<false>(max_index); 
        }
        let node = self.node;
        let node = Rc::try_unwrap(node).ok().unwrap(); 
        let node = node.into_inner(); 
        node.value 
    }
}

// impl <'a, T: Ord> Drop for PeekMut<'a, T> {
//     fn drop(&mut self) {
//         let borrow = self.node.borrow(); 
//         let min_index = borrow.min_index; 
//         let max_index = borrow.max_index; 
//         drop(borrow); 
//         self.bi_heap.bubble_down::<true>(min_index); 
//         self.bi_heap.bubble_up::<true>(min_index); 
//         self.bi_heap.bubble_down::<false>(max_index); 
//         self.bi_heap.bubble_up::<false>(max_index); 
//     }
// }