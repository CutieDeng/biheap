use std::{error::Error, fmt::Display};

use super::*; 

impl <T: Ord> BiHeap<T> {
    pub fn max_handle(&self) -> Option<Handle<T>> {
        let borrow = self.0.borrow(); 
        let slice = borrow.views(); 
        let slice = &slice[1]; 
        if slice.is_empty() {
            None
        } else {
            let node_ref = Rc::downgrade(&slice[0]); 
            let heap_ref = Rc::downgrade(&self.0); 
            Some(Handle {
                node_ref, 
                heap_ref, 
            })
        }
    }  
    pub fn min_handle(&self) -> Option<Handle<T>> {
        let borrow = self.0.borrow(); 
        let slice = borrow.views(); 
        let slice = &slice[0]; 
        if slice.is_empty() {
            None
        } else {
            let node_ref = Rc::downgrade(&slice[0]); 
            let heap_ref = Rc::downgrade(&self.0); 
            Some(Handle {
                node_ref, 
                heap_ref, 
            })
        } 
    }
}

#[derive(Debug)]
pub enum ViewErr {
    MismatchHeap, 
    MissValue, 
}

impl Error for ViewErr {} 

impl Display for ViewErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl <T: Ord> BiHeap<T> {
    pub fn as_view(&self, handle: &Handle<T>) -> Result<&T, ViewErr> { 
        let weak_ref = Rc::downgrade(&self.0); 
        if !Weak::ptr_eq(&weak_ref, &handle.heap_ref) {
            return Err(ViewErr::MismatchHeap);  
        }
        let value = handle.node_ref.upgrade().ok_or(ViewErr::MissValue)?;
        let value = unsafe { value.try_borrow_unguarded() }; 
        let value = value.unwrap(); 
        let value = &value.value; 
        let value = unsafe { &*(value as *const T) }; 
        Ok(value) 
    } 
}

impl <T: Ord> BiHeap<T> {
    pub fn as_view_mut<'a> (&'a mut self, handle: &'_ Handle<T>) -> Result<ViewMut<'a, T>, ViewErr> {
        let weak_ref = Rc::downgrade(&self.0); 
        if !Weak::ptr_eq(&weak_ref, &handle.heap_ref) {
            return Err(ViewErr::MismatchHeap);  
        } 
        let value = handle.node_ref.upgrade().ok_or(ViewErr::MissValue)?; 
        let view = ViewMut {
            bi_heap: self, 
            node: Some(value), 
        }; 
        Ok(view)
    }
}