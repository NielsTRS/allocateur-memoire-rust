//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use std::ptr;
use std::assert;
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::mem_space;

pub struct MemFreeBlock {
    next: Option<Box<MemFreeBlock>>,  // Pointer to the next free block (linked list)
    size: usize,                      // Size of the free block
}

pub type MemFitFunction = fn(first_free_block: &MemFreeBlock, wanted_size: usize) -> Option<&MemFreeBlock>;

lazy_static! {
    static ref FB: Mutex<MemFreeBlock> = Mutex::new(MemFreeBlock {
        next: None,
        size: mem_space::MEMORY_SIZE,
    });
}

//-------------------------------------------------------------
// mem_init
//-------------------------------------------------------------
/// Initialize the memory allocator.
/// If already initialized, it will re-init.
/// // You can reinitialize it here if needed
pub fn mem_init() {
    let mut fb = FB.lock().unwrap();
    fb.next = None;
    fb.size = mem_space::MEMORY_SIZE;
}

pub fn get_fb() -> std::sync::MutexGuard<'static, MemFreeBlock> {
    FB.lock().unwrap()
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
