use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::bivec::BiVec;

pub struct BiHeap <T: Ord> (pub(crate) Rc<RefCell<BiVec<Rc<RefCell<Node<T>>>>>>); 

pub struct Node <T> {
    pub(crate) value: T, 
    pub(crate) min_index: usize, 
    pub(crate) max_index: usize, 
}

pub type Handle<T> = Indexer<T>; 

pub struct Indexer <T> {
    pub(crate) node_ref: Weak<RefCell<Node<T>>>, 
    pub(crate) heap_ref: Weak<RefCell<BiVec<Rc<RefCell<Node<T>>>>>>, 
}

pub struct ViewMut <'a, T: Ord> {
    pub(crate) bi_heap: &'a mut BiHeap<T>,
    pub(crate) node: Rc<RefCell<Node<T>>>,
}

mod construct;
mod push;
mod bubble;
mod handle;
mod view;
mod peek;
mod pop;
mod property;