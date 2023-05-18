use std::mem::swap;
use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::cmp::Ordering; 

pub struct Node <T> {
    value: T, 
    minimum_index: usize, 
    maximum_index: usize, 
}

impl <T> Node<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
} 

pub struct BiHeap <T: Ord> {
    min_heap: Vec<Rc<RefCell<Node<T>>>>, 
    max_heap: Vec<Rc<RefCell<Node<T>>>>, 
}

impl <T: Ord> BiHeap<T> {
    pub fn new() -> Self {
        BiHeap {
            min_heap: Vec::new(), 
            max_heap: Vec::new(), 
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        BiHeap {
            min_heap: Vec::with_capacity(capacity), 
            max_heap: Vec::with_capacity(capacity), 
        }  
    }
}

fn minimin_index_mut <T> (node: &mut Node<T>) -> &mut usize {
    &mut node.minimum_index
} 

fn maximax_index_mut <T> (node: &mut Node<T>) -> &mut usize {
    &mut node.maximum_index
} 

impl <T: Ord> BiHeap<T> {
    fn bubble_up <const IS_MIN: bool> (&mut self, index: usize) {
        if IS_MIN {
            assert!( index < self.min_heap.len() ); 
        } else {
            assert!( index < self.max_heap.len() );  
        }
        let heap = if IS_MIN { &mut self.min_heap } else { &mut self.max_heap }; 
        let accessor = if IS_MIN { minimin_index_mut } else { maximax_index_mut }; 
        let slice = &mut heap[..]; 
        let mut index = index; 
        while index != 0 {
            let pindex = (index - 1) / 2; 
            let (parent, child) = slice.split_at_mut(index); 
            let (parent, child) = (&mut parent[pindex], &mut child[0]); 
            let cmp = Ord::cmp(&parent.borrow().value, &child.borrow().value); 
            match cmp {
                Ordering::Less => {
                    if !IS_MIN {
                        *accessor(&mut parent.borrow_mut()) = index;  
                        *accessor(&mut child.borrow_mut()) = pindex; 
                        swap(parent, child); 
                    } else {
                        break; 
                    }
                }
                Ordering::Greater => {
                    if IS_MIN {
                        *accessor(&mut parent.borrow_mut()) = index;  
                        *accessor(&mut child.borrow_mut()) = pindex; 
                        swap(parent, child);  
                    } else {
                        break; 
                    }
                }
                Ordering::Equal => break, 
            }
            index = pindex; 
        }
    }
}

impl <T: Ord> BiHeap<T> {
    pub fn push(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node {
            value, 
            minimum_index: self.min_heap.len(), 
            maximum_index: self.max_heap.len(), 
        })); 
        self.min_heap.push(Rc::clone(&node)); 
        self.max_heap.push(Rc::clone(&node)); 
        drop(node); 
        self.bubble_up::<true>(self.min_heap.len() - 1);
        self.bubble_up::<false>(self.max_heap.len() - 1);
    }
} 

impl <T: Ord> BiHeap<T> {
    pub fn peek_min <'a> (&'a self) -> Option<Ref<'a, Node<T>>> {
        let r = self.min_heap.get(0); 
        r.map(|node| node.borrow()) 
    }
    pub fn peek_max <'a> (&'a self) -> Option<Ref<'a, Node<T>>> {
        let r = self.max_heap.get(0); 
        r.map(|node| node.borrow()) 
    }
}
 
impl <T: Ord> BiHeap<T> {
    pub fn pop_min(&mut self) -> bool {
        if self.min_heap.len() == 1 {
            self.min_heap.pop(); 
            self.max_heap.pop(); 
            return true; 
        }
        let p = self.min_heap.pop(); 
        let Some(mut p) = p else {
            return false;  
        }; 
        swap(&mut p, &mut self.min_heap[0]); 
        let max_index = p.borrow().maximum_index; 
        let sr = self.max_heap.swap_remove(max_index); 
        self.bubble_down::<true>(0); 
        self.bubble_down::<false>(max_index);
        drop(sr); 
        return true; 
    }
    pub fn pop_max(&mut self) -> Option<T> {
        todo!()
    } 
}

impl <T: Ord> BiHeap<T> {
    fn bubble_down<const IS_MIN: bool> (&mut self, index: usize) {
        if IS_MIN {
            assert!( index < self.min_heap.len() ); 
        } else {
            assert!( index < self.max_heap.len() );   
        }
        let heap = if IS_MIN { &mut self.min_heap } else { &mut self.max_heap }; 
        let accessor = if IS_MIN { minimin_index_mut } else { maximax_index_mut }; 
        let slice = &mut heap[..]; 
        let mut index = index; 
        loop {
            let lindex = 2 * index + 1; 
            if lindex >= slice.len() {
                break; 
            } 
            let (parent, children) = slice.split_at_mut(lindex); 
            let (parent, children) = (&mut parent[index], &mut children[..]); 
            let (left, children) = children.split_first_mut().unwrap(); 
            let right = children.split_first_mut().map(|a| a.0); 
            let mut swap_index = lindex; 
            let mut swap_node = left; 
            if let Some(right) = right {
                let cmp = Ord::cmp(&swap_node.borrow().value, &right.borrow().value); 
                match cmp {
                    Ordering::Less => {
                        if !IS_MIN {
                            swap_index = lindex + 1; 
                            swap_node = right; 
                        }
                    }
                    Ordering::Greater => {
                        if IS_MIN {
                            swap_index = lindex + 1; 
                            swap_node = right; 
                        }
                    }
                    Ordering::Equal => (), 
                }
            }
            let cmp = Ord::cmp(&parent.borrow().value, &swap_node.borrow().value); 
            match cmp {
                Ordering::Less => {
                    if !IS_MIN {
                        *accessor(&mut parent.borrow_mut()) = swap_index;  
                        *accessor(&mut swap_node.borrow_mut()) = index; 
                        swap(parent, swap_node); 
                    } else {
                        break; 
                    }
                }
                Ordering::Greater => {
                    if IS_MIN {
                        *accessor(&mut parent.borrow_mut()) = swap_index;  
                        *accessor(&mut swap_node.borrow_mut()) = index; 
                        swap(parent,  swap_node);  
                    } else {
                        break; 
                    }
                }
                Ordering::Equal => break, 
            }
            index = swap_index; 
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
