use super::BiHeap;

impl <T: Ord> BiHeap<T> {
    pub fn len(&self) -> usize {
        self.0.borrow().len() 
    }
}

impl <T: Ord + std::fmt::Debug> BiHeap<T> {
    #[cfg(test)]
    pub fn debug(&self) {
        let borrow = self.0.borrow(); 
        let [v1, v2] = borrow.views(); 
        let iter = v1.iter().enumerate(); 
        eprintln!("min slice");
        for (i, v) in iter {
            let min_index = v.borrow().min_index; 
            let max_index = v.borrow().max_index; 
            dbg!((i, min_index, max_index, &v.borrow().value));
        }
        eprintln!("---\nmax slice");
        let iter = v2.iter().enumerate(); 
        for (i, v) in iter {
            let min_index = v.borrow().min_index; 
            let max_index = v.borrow().max_index; 
            dbg!((i, min_index, max_index, &v.borrow().value));
        } 
    }
    #[cfg(test)]
    pub fn check(&self) {
        use std::rc::Rc;

        let borrow = self.0.borrow(); 
        let [v1, v2] = borrow.views(); 
        let iter = v1.iter().enumerate(); 
        for (i, v) in iter {
            let min_index = v.borrow().min_index; 
            let max_index = v.borrow().max_index; 
            assert_eq!(i, min_index); 
            let max = &v2[max_index]; 
            let eq = Rc::ptr_eq(&v, &max); 
            assert!(eq); 
        } 

    }
}