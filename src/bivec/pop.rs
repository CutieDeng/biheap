use super::BiVec;

impl <T> BiVec<T> {
    /// pop the last element of the BiVec 
    pub fn pop(&mut self) -> Option<(T, T)> {
        if self.len == 0 {
            return None; 
        }
        let (a, b); 
        self.len -= 1; 
        unsafe {
            a = self.contents[0].add(self.len).read(); 
            b = self.contents[1].add(self.len).read(); 
        } 
        Some((a, b)) 
    } 
}

impl <T> BiVec<T> {
    pub fn swap_remove(&mut self, first_index: usize, second_index: usize) -> Option<(T, T)> {
        if first_index >= self.len || second_index >= self.len {
            return None; 
        }
        let (p1, p2); 
        let (a, b); 
        self.len -= 1; 
        unsafe {
            p1 = self.contents[0].add(first_index) as *mut T; 
            a = p1.read(); 
            p1.write( self.contents[0].add(self.len).read() ); 
            p2 = self.contents[1].add(second_index) as *mut T; 
            b = p2.read(); 
            p2.write( self.contents[1].add(self.len).read() ); 
        }
        Some((a, b)) 
    } 
}