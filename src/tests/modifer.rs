use super::*; 

#[test] 
fn middle_to_maximum() {
    let mut bh = BiHeap::new(); 
    bh.push(1); 
    let take = bh.push(2); 
    bh.push(3); 
    bh.push(4); 
    let max1 = bh.peek_max(); 
    assert_eq!(max1, Some(&4)); 
    let mut view = bh.as_view_mut(&take).unwrap(); 
    view.set(5); 
    let max2 = bh.peek_max(); 
    assert_eq!(max2, Some(&5)); 
}

#[test] 
fn middle_to_minimum() {
    let mut bh = BiHeap::new(); 
    bh.push(1); 
    let take = bh.push(2); 
    bh.push(3); 
    bh.push(4); 
    let min1 = bh.peek_min(); 
    assert_eq!(min1, Some(&1)); 
    let mut view = bh.as_view_mut(&take).unwrap(); 
    view.set(0); 
    let min2 = bh.peek_min(); 
    assert_eq!(min2, Some(&0));  
}