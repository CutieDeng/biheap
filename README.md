# BiHeap 

BiHeap is a implementation of heap structure, to maintain the extreme values of a list of data. 

## Structure 

### BiHeap 

`BiHeap` is the core data structure to control the two heaps. 
Any data follows by `Ord` trait can be the type parameter of the BiHeap generic struct. 

### Indexer 

The `Handle` is a reference to the data in the heap. 
Actually it doesn't describe the owner of the data, but the position of the data in the heap. 
It means the data it points out would be 'miss', if you remove it from the heap. 

### PeekMut

The `ViewMut` is a mutable reference to the data in the heap. 
It can be used to modify the data in the heap, peek it, reset it, or remove it. 

## Latest Version Log 

- Add `iter` method. 
- Add `peek_max_mut`, `peek_min_mut` methods. 

## Others 

If you have some questions, welcome on https://github.com/CutieDeng/biheap/issues. 