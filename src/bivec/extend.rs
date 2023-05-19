
use std::ptr;

use super::BiVec;

impl <T> BiVec<T> {
    pub fn reserve(&mut self, capacity: usize) {
        if self.capacity >= capacity {
            return; 
        } 
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap(); 
        let ptr = unsafe { std::alloc::alloc(layout) } as *const T; 
        let ptr2 = unsafe { std::alloc::alloc(layout) } as *const T; 
        unsafe {
            ptr::copy_nonoverlapping(self.contents[0], ptr as *mut T, self.len); 
            ptr::copy_nonoverlapping(self.contents[1], ptr2 as *mut T, self.len);  
        }
        if self.capacity != 0 {
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap(); 
            unsafe {
                std::alloc::dealloc(self.contents[0] as *mut u8, layout); 
                std::alloc::dealloc(self.contents[1] as *mut u8, layout); 
            }
        }
        self.contents[0] = ptr; 
        self.contents[1] = ptr2; 
        self.capacity = capacity; 
        return ; 
    }
}