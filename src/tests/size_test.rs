use crate::core::*; 

#[test] 
fn empty() {
    let heap: BiHeap<i32> = BiHeap::new(); 
    assert_eq!(heap.len(), 0); 
    heap.debug_check();
} 

#[test] 
fn one() {
    let mut heap = BiHeap::new(); 
    heap.debug_check();
    heap.push(1); 
    assert_eq!(heap.len(), 1);  
    heap.debug_check();
}

#[test] 
fn two() {
    let mut heap = BiHeap::new(); 
    heap.debug_check();
    heap.push(1); 
    heap.debug_check();
    heap.push(2); 
    heap.debug_check();
    assert_eq!(heap.len(), 2); 
}

#[test] 
fn three() {
    let mut heap = BiHeap::new(); 
    heap.push(1); 
    heap.push(2); 
    heap.push(3); 
    assert_eq!(heap.len(), 3); 
    // pop max 
    heap.max().unwrap().as_view().take().unwrap(); 
    assert_eq!(heap.len(), 2); 
    // pop min 
    heap.min().unwrap().as_view().take().unwrap(); 
    assert_eq!(heap.len(), 1); 
    // pop max 
    heap.max().unwrap().as_view().take().unwrap(); 
    assert_eq!(heap.len(), 0); 
    let p = heap.min(); 
    assert!(p.is_none());
    assert_eq!(heap.len(), 0); 
} 