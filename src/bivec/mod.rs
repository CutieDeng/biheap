use std::marker::PhantomData;

pub struct BiVec <T> {
    pub(crate) contents: [*const T; 2], 
    pub(crate) len: usize, 
    pub(crate) capacity: usize, 
    flag: PhantomData<T>, 
}

mod construct; 
mod extend;
mod push;
mod pop;
mod property; 

pub mod view;
pub mod order;