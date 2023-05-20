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

pub struct PeekMut <'a, T: Ord> {
    pub(crate) bi_heap: &'a mut BiHeap<T>,
    // Actaully, it should always be Some, or in my consumming method it will be None, only. 
    pub(crate) node: Option<Rc<RefCell<Node<T>>>>, 
}

pub type ViewMut<'a, T> = PeekMut<'a, T>; 

mod construct;
mod push;
mod bubble;
mod handle;
mod view;
mod peek;
mod pop;
mod property;
mod clone;
mod debug;
mod default;
mod extend;
mod from;
mod iter;
mod into;