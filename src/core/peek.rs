use super::*; 

impl <T: Ord> BiHeap<T> {
    pub fn peek_min(&self) -> Option<&T> {
        let handle = self.min_handle()?; 
        let value = self.as_view(&handle).unwrap(); 
        Some(value) 
    }
    pub fn peek_max(&self) -> Option<&T> {
        let handle = self.max_handle()?; 
        let value = self.as_view(&handle).unwrap(); 
        Some(value) 
    } 
    pub fn pop_min(&mut self) -> Option<T> {
        let handle = self.min_handle()?; 
        let value = self.as_view_mut(&handle).unwrap(); 
        Some(value.pop()) 
    }
    pub fn pop_max(&mut self) -> Option<T> {
        let handle = self.max_handle()?; 
        let value = self.as_view_mut(&handle).unwrap(); 
        Some(value.pop()) 
    } 
}