use super::*; 

#[test] 
fn empty() {
    let heap: BiHeap<i32> = BiHeap::new(); 
    assert_eq!(heap.size(), 0); 
} 

#[test] 
fn one() {
    let mut heap = BiHeap::new(); 
    heap.push(1); 
    assert_eq!(heap.size(), 1);  
}

#[test] 
fn two() {
    let mut heap = BiHeap::new(); 
    heap.push(1); 
    heap.push(2); 
    assert_eq!(heap.size(), 2); 
}

#[test] 
fn three() {
    let mut heap = BiHeap::new(); 
    heap.push(1); 
    heap.push(2); 
    heap.push(3); 
    assert_eq!(heap.size(), 3); 
    heap.pop_min(); 
    assert_eq!(heap.size(), 2); 
    heap.pop_max(); 
    assert_eq!(heap.size(), 1); 
    heap.pop_min(); 
    assert_eq!(heap.size(), 0); 
    let p = heap.pop_max(); 
    assert_eq!(p, None); 
    assert_eq!(heap.size(), 0); 
} 