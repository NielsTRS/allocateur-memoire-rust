//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use std::ptr;
use std::assert;

//-------------------------------------------------------------
// mem_init
//-------------------------------------------------------------
/// Initialize the memory allocator.
/// If already initialized, it will re-init.
pub fn mem_init() {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
}

//-------------------------------------------------------------
// mem_alloc
//-------------------------------------------------------------
/// Allocate a block of the given size.
pub fn mem_alloc(size: usize) -> *mut u8 {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
    ptr::null_mut()
}

//-------------------------------------------------------------
// mem_get_size
//-------------------------------------------------------------
pub fn mem_get_size(zone: *mut u8) -> usize {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
    0
}

//-------------------------------------------------------------
// mem_free
//-------------------------------------------------------------
/// Free an allocated block.
pub fn mem_free(zone: *mut u8) {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
}

//-------------------------------------------------------------
// Iterator (parcours) over the content of the allocator
// mem_show
//-------------------------------------------------------------
pub fn mem_show(print: fn(*mut u8, usize, bool)) {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
}

//-------------------------------------------------------------
// mem_fit
//-------------------------------------------------------------
pub fn mem_set_fit_handler(mff: fn(*mut MemFreeBlock, usize) -> *mut MemFreeBlock) {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
}

//-------------------------------------------------------------
// Allocation strategies
//-------------------------------------------------------------
pub struct MemFreeBlock {
    // Assuming this structure has fields to hold necessary information.
    // Define fields based on what you need
    size: usize,
    // More fields based on allocator requirements
}

//-------------------------------------------------------------
// First Fit Strategy
//-------------------------------------------------------------
pub fn mem_first_fit(first_free_block: *mut MemFreeBlock, wanted_size: usize) -> *mut MemFreeBlock {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
    ptr::null_mut()
}

//-------------------------------------------------------------
// Best Fit Strategy
//-------------------------------------------------------------
pub fn mem_best_fit(first_free_block: *mut MemFreeBlock, wanted_size: usize) -> *mut MemFreeBlock {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
    ptr::null_mut()
}

//-------------------------------------------------------------
// Worst Fit Strategy
//-------------------------------------------------------------
pub fn mem_worst_fit(first_free_block: *mut MemFreeBlock, wanted_size: usize) -> *mut MemFreeBlock {
    // TODO: implement
    assert!(false, "NOT IMPLEMENTED !");
    ptr::null_mut()
}
