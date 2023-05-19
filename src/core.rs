
#[cfg(not(threadsafe))]
pub type Shared<T> = std::rc::Rc<std::cell::RefCell<T>>; 
#[cfg(threadsafe)]
type Shared<T> = std::sync::Arc<std::sync::Mutex<T>>; 

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

