//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use lazy_static::lazy_static;
use std::assert;
use std::ptr;
use std::sync::Mutex;
use crate::mem_space::*;

pub struct MemFreeBlock {
    next: Option<Box<MemFreeBlock>>, // Pointer to the next free block (linked list)
    size: usize,                     // Size of the free block
}

pub struct MemMetaBlock {
    size: usize, // Size of the block
}

lazy_static! {
    static ref FB: Mutex<MemFreeBlock> = Mutex::new(MemFreeBlock {
        next: None,
        size: 0,
    });
}

pub fn get_fb() -> std::sync::MutexGuard<'static, MemFreeBlock> {
    FB.lock().unwrap()
}

//-------------------------------------------------------------
// mem_init
//-------------------------------------------------------------
/// Initialize the memory allocator.
/// If already initialized, it will re-init.
/// // You can reinitialize it here if needed
pub fn mem_init() {
    let mut fb = get_fb();
    fb.next = None;
    fb.size = mem_space_get_size();
    mem_set_fit_handler(mem_first_fit);
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
    let fb = get_fb();
    let mut current_block = Some(&*fb);

    while let Some(block) = current_block {
        let block_addr = block as *const MemFreeBlock as *mut u8;
        print(block_addr, block.size, true);
        current_block = block.next.as_deref();
    }
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
