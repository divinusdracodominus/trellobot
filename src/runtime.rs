use core::alloc::{GlobalAlloc, Layout, alloc};
use core::ptr::null_mut;

pub struct MemoryAllocation {
    start: *mut u8,
    length: usize,
}

pub struct HeapAllocator {
    count: usize,
    capacity: usize,
    data: *mut MemoryAllocation,
}

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut () }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocate]
static A: HeapAllocator = HeapAllocator {count: 0, capacity: 0, data: null_mut() }