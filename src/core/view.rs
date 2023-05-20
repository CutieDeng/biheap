use super::*; 

impl <T: Ord> BiHeap<T> {
    /// Returns the minimum element reference of the heap. 
    /// 
    /// # Examples 
    /// ``` 
    /// use biheap::BiHeap; 
    /// let mut be = BiHeap::new(); 
    /// be.push(1); 
    /// be.push(2); 
    /// be.push(3); 
    /// let min = be.peek_min().unwrap(); 
    /// assert_eq!(min, &1); 
    /// ``` 
    pub fn peek_min(&self) -> Option<&T> {
        let handle = self.min_handle()?; 
        let value = self.peek(&handle).unwrap(); 
        Some(value) 
    }
    /// Returns the maximum element reference of the heap. 
    /// 
    /// # Examples
    /// ``` 
    /// use biheap::BiHeap; 
    /// let mut be = BiHeap::new(); 
    /// be.push(1); 
    /// be.push(2); 
    /// be.push(3); 
    /// let max = be.peek_max().unwrap(); 
    /// assert_eq!(max, &3); 
    /// ``` 
    pub fn peek_max(&self) -> Option<&T> {
        let handle = self.max_handle()?; 
        let value = self.peek(&handle).unwrap(); 
        Some(value) 
    } 
    /// Pops the minimum element value of the heap. 
    /// 
    /// # Examples 
    /// ```
    /// use biheap::BiHeap;
    /// let mut be = BiHeap::new();
    /// be.push(1);
    /// be.push(2);
    /// be.push(3);
    /// let min = be.pop_min().unwrap();
    /// assert_eq!(min, 1);
    /// let min = be.pop_min().unwrap(); 
    /// assert_eq!(min, 2); 
    /// ``` 
    pub fn pop_min(&mut self) -> Option<T> {
        let handle = self.min_handle()?; 
        let value = self.peek_mut(&handle).unwrap(); 
        Some(value.pop()) 
    }
    /// Pops the maximum element value of the heap. 
    /// 
    /// # Examples
    /// ```
    /// use biheap::BiHeap;
    /// let mut be = BiHeap::new();
    /// be.push(1);
    /// be.push(2);
    /// be.push(3);
    /// let max = be.pop_max().unwrap();
    /// assert_eq!(max, 3);
    /// let max = be.pop_max().unwrap();
    /// assert_eq!(max, 2);
    /// ```
    pub fn pop_max(&mut self) -> Option<T> {
        let handle = self.max_handle()?; 
        let value = self.peek_mut(&handle).unwrap(); 
        Some(value.pop()) 
    } 
}
