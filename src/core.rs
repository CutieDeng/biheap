use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::bivec::BiVec;

pub struct BiHeap <T: Ord> (pub(crate) Rc<RefCell<BiVec<Rc<RefCell<Node<T>>>>>>); 

pub struct Node <T> {
    pub(crate) value: T, 
    pub(crate) min_index: usize, 
    pub(crate) max_index: usize, 
}

pub struct Handle <T> {
    pub(crate) node_ref: Weak<RefCell<Node<T>>>, 
    pub(crate) heap_ref: Weak<RefCell<BiVec<Rc<RefCell<Node<T>>>>>>, 
}

mod construct;
mod push;
mod bubble;