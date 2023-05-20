use std::{rc::Rc, cell::RefCell};

use crate::bivec::BiVec;

pub struct BiHeap <T: Ord> (pub(crate) Rc<RefCell<BiVec<T>>>); 

mod construct;