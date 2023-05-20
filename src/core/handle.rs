use super::*; 

impl <T: Ord> BiHeap<T> {
    /// Returns the maximum element handle of the heap. 
    /// 
    /// # Examples 
    /// ``` 
    /// use biheap::BiHeap; 
    /// let mut be = BiHeap::new(); 
    /// be.push(1); 
    /// let handle = be.max_handle(); 
    /// ``` 
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
    /// Returns the minimum element handle of the heap. 
    /// 
    /// # Examples 
    /// ``` 
    /// use biheap::BiHeap; 
    /// let mut be = BiHeap::new(); 
    /// be.push(1); 
    /// let handle = be.min_handle(); 
    /// ``` 
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
