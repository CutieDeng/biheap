use std::marker::PhantomData;

pub struct BiVec <T> {
    contents: [*const T; 2], 
    len: usize, 
    capacity: usize, 
    flag: PhantomData<T>, 
}

mod construct; 
mod extend;
mod push;
mod pop;
pub mod view; 
