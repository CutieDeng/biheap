use crate::v2::*; 

#[test]
fn middle_pop() {
    let mut bh = BiHeap::new(); 
    bh.push(1); 
    bh.push(2); 
    let value2 = bh.max().unwrap().as_view(); 
    bh.push(3); 
    assert_eq!( bh.len(), 3 ); 
    assert_eq!( value2.take().unwrap(), 2 ); 
    assert_eq!( bh.len(), 2 ); 
    assert_eq!( bh.pop_max().unwrap(), 3 ); 
    assert_eq!( bh.len(), 1 ); 
    assert_eq!( bh.pop_max().unwrap(), 1 ); 
    assert_eq!( bh.len(), 0 ); 
}

#[test] 
fn loss_pop() {
    let mut bh = BiHeap::new(); 
    bh.push(1); 
    let value = bh.min().unwrap().as_view(); 
    drop(bh);  
    let take = value.take();
    assert!( take.is_err() ); 
}

#[test] 
fn missing_pop() {
    let mut bh = BiHeap::new(); 
    bh.push(1); 
    let value = bh.min().unwrap().as_view(); 
    let value2 = bh.min().unwrap().as_view(); 
    let take = value.take(); 
    assert_eq!( take.unwrap(), 1 ); 
    let take = value2.take(); 
    assert!( take.is_err() );  
}

#[test] 
fn missing_pop2() {
    let mut bh = BiHeap::new(); 
    bh.push(1); 
    let value = bh.min().unwrap().as_view(); 
    let v = bh.pop_max().unwrap(); 
    assert_eq!( v, 1 ); 
    let take = value.take(); 
    assert!( take.is_err() ); 
}