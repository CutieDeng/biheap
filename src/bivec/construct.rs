use std::alloc::Layout;
use std::ptr;
use std::marker::PhantomData;

use super::BiVec;

impl <T> BiVec<T> {
    pub fn new() -> Self {
        BiVec {
            contents: [ptr::null(), ptr::null()], 
            len: 0, 
            capacity: 0, 
            flag: PhantomData, 
        }
    } 
}

impl <T> BiVec<T> {
    pub fn with_reserve(capacity: usize) -> Self {
        let layout : Layout = Layout::array::<T>(capacity).unwrap(); 
        let ptr = unsafe { std::alloc::alloc(layout) } as *const T;  
        let ptr2 = unsafe { std::alloc::alloc(layout) } as *const T; 
        BiVec {
            contents: [ptr, ptr2], 
            len: 0, 
            capacity: capacity, 
            flag: PhantomData, 
        } 
    }
}

impl <T> Drop for BiVec<T> {
    fn drop(&mut self) {
        for ele in self.contents {
            for i in 0..self.len {
                unsafe { 
                    ptr::drop_in_place(
                        ele.add(i) as *mut T 
                    ); 
                }
            } 
        }
        let layout : Layout = Layout::array::<T>(self.capacity).unwrap(); 
        unsafe { std::alloc::dealloc(self.contents[0] as *mut u8, layout) }; 
        unsafe { std::alloc::dealloc(self.contents[1] as *mut u8, layout) }; 
    } 
}
