use crate::core::{Weak, RawBiVec, RawNode};

pub struct View<T: Ord> {
    pub (crate) origin_heap: Weak<RawBiVec<T>>, 
    pub (crate) raw_node: Weak<RawNode<T>>,
} 

#[derive(Debug)]
pub enum TakeErr {
    HeapDroped, 
    NodeMissing,
}

impl <T: Ord> View <T> {
    pub fn take(self) -> Result<T, TakeErr> {
        let origin_heap = self.origin_heap.upgrade().ok_or(TakeErr::HeapDroped)?; 
        let raw_node = self.raw_node.upgrade().ok_or(TakeErr::NodeMissing)?; 
        let mut origin_heap = origin_heap.borrow_mut(); 
        let (min_index, max_index); 
        {
            let raw_node = raw_node.borrow(); 
            min_index = raw_node.min_index; 
            max_index = raw_node.max_index; 
        }
        let val_node = origin_heap.min[min_index].clone(); 
        origin_heap.min.swap_remove(min_index); 
        origin_heap.max.swap_remove(max_index); 
        let _ = origin_heap.bubble_down::<true>(min_index); 
        let _ = origin_heap.bubble_pop::<true>(min_index);
        let _ = origin_heap.bubble_down::<false>(max_index);
        let _ = origin_heap.bubble_pop::<false>(max_index); 
        let val = std::rc::Rc::try_unwrap(val_node).ok().unwrap().into_inner();
        Ok(val.data) 
    }
}