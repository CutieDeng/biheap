use std::mem::swap;

use super::*; 

impl <T: Ord> BiHeap<T> {
    pub(crate) fn bubble_up<const IS_MIN_FIRST: bool>(&mut self, index: usize) {
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

impl <T: Ord> BiHeap<T> {
    pub(crate) fn bubble_down<const IS_MIN_FIRST: bool>(&mut self, index: usize) {
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
            let left_index = index * 2 + 1; 
            if left_index >= slice.len() {
                break; 
            } 
            let (mut split1, mut split2) = slice.split_at_mut(left_index); 
            let this = &mut split1[index]; 
            let (mut left, mut others) = split2.split_first_mut().unwrap(); 
            let right = others.first_mut(); 
            let select; 
            let cell_ref; 
            let should_swap; 
            if let Some(right) = right {
                let left_ref = left.borrow(); 
                let right_ref = right.borrow(); 
                if IS_MIN_FIRST {
                    if left_ref.value < right_ref.value {
                        select = left_index; 
                        drop(left_ref); 
                        cell_ref = left; 
                    } else {
                        select = left_index + 1;  
                        drop(right_ref); 
                        cell_ref = right; 
                    }
                } else {
                    if left_ref.value > right_ref.value {
                        select = left_index; 
                        drop(left_ref); 
                        cell_ref = left; 
                    } else {
                        select = left_index + 1; 
                        drop(right_ref); 
                        cell_ref = right; 
                    } 
                }
            } else {
                select = left_index; 
                cell_ref = left; 
            }
            if IS_MIN_FIRST {
                let this = this.borrow(); 
                let cell = cell_ref.borrow(); 
                should_swap = this.value > cell.value; 
            } else {
                let this = this.borrow(); 
                should_swap = this.value < cell_ref.borrow().value; 
            }
            if !should_swap {
                break; 
            }
            swap(this, cell_ref); 
            let mut this = this.borrow_mut(); 
            let mut cell = cell_ref.borrow_mut(); 
            if IS_MIN_FIRST {
                this.min_index = select; 
                cell.min_index = index; 
            } else {
                this.max_index = select; 
                cell.max_index = index; 
            } 
            index = select; 
        }
    }
}