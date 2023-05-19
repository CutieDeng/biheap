#[cfg(not(threadsafe))]
pub type Shared<T> = std::rc::Rc<std::cell::RefCell<T>>; 
#[cfg(threadsafe)]
pub type Shared<T> = std::sync::Arc<std::sync::Mutex<T>>; 

#[cfg(not(threadsafe))] 
pub type Weak<T> = std::rc::Weak<std::cell::RefCell<T>>; 
#[cfg(threadsafe)] 
pub type Weak<T> = std::sync::Weak<std::sync::Mutex<T>>; 

/// Data simple structure to store the data. 
/// 
/// # Fields 
/// data: T, the data stored  
/// min_index: usize, the index of the minimum value in the data 
/// max_index: usize, the index of the maximum value in the data 
/// 
/// # Accessible 
/// Never accessible outside the crate. 
pub struct RawNode <T> {
    pub data: T, 
    pub min_index: usize, 
    pub max_index: usize, 
}

/// Data structure to store the two heap 
pub struct RawBiVec <T> {
    pub max: Vec<Shared<RawNode<T>>>, 
    pub min: Vec<Shared<RawNode<T>>>, 
}

/// Data structure to store the two heap 
/// 
/// # Implementation 
/// Actually, it is a wrapper of RawBiVec<T>. 
pub struct BiHeap <T: Ord> {
    pub(crate) bi_vec: Shared<RawBiVec<T>>, 
}

impl <T: Ord> BiHeap<T> {
    pub fn len(&self) -> usize {
        let bi_vec = self.bi_vec.borrow(); 
        bi_vec.max.len() 
    }
}

impl <T: Ord> BiHeap<T> {
    #[cfg(test)]
    pub(crate) fn debug_check(&self) {
        let bi_vec = self.bi_vec.borrow(); 
        let max_len = bi_vec.max.len(); 
        let min_len = bi_vec.min.len(); 
        assert_eq!(max_len, min_len); 
        for i in 0..max_len {
            let min = &bi_vec.min[i]; 
            let minr = min.borrow(); 
            assert_eq!(minr.min_index, i); 
            let max_i = min.borrow().max_index; 
            let max = &bi_vec.max[max_i]; 
            assert!(std::rc::Rc::ptr_eq(max, min));
        } 
    }
}