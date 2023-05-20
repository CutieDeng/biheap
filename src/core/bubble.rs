use std::mem::swap;

use super::*; 

impl <T: Ord> BiHeap<T> {
    pub fn bubble_up<const IS_MIN_FIRST: bool>(&mut self, index: usize) {
        let mut borrow = self.0.borrow_mut(); 
        debug_assert!(index < borrow.len());
        let mut slice = borrow.views_mut(); 
        let slice = 
            if IS_MIN_FIRST {
                &mut slice[0] 
            } else {
                &mut slice[1] 
            };
        let mut index = index; 
        loop {
            if index == 0 {
                break; 
            }
            let parent_index = (index - 1) / 2; 
            let parent = slice.split_at_mut(index); 
            let this = parent.1.first_mut().unwrap(); 
            let parent = &mut parent.0[parent_index]; 
            let should_swap; 
            {
                let this = this.borrow(); 
                let parent = parent.borrow(); 
                should_swap = 
                    if IS_MIN_FIRST {
                        this.value < parent.value
                    } else {
                        this.value > parent.value
                    }; 
            }
            if should_swap {
                swap(this, parent); 
                let mut this = this.borrow_mut(); 
                let mut parent = parent.borrow_mut(); 
                if IS_MIN_FIRST {
                    this.min_index = parent_index; 
                    parent.min_index = index; 
                } else {
                    this.max_index = parent_index; 
                    parent.max_index = index;  
                }
                index = parent_index; 
            } else {
                break; 
            }
        }
    }
}