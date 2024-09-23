//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use std::assert;
use std::ptr;
use crate::mem_space::*;

static mut FIT_HANDLER: Option<fn(*mut MemFreeBlock, usize) -> *mut MemFreeBlock> = None;

pub struct MemFreeBlock {
    pub next: Option<Box<MemFreeBlock>>, // Pointer to the next free block (linked list)
    pub size: usize,                     // Size of the free block
}

impl MemFreeBlock {

    //-------------------------------------------------------------
    // mem_init
    //-------------------------------------------------------------
    /// Initialize the memory allocator.
    /// If already initialized, it will re-init.
    /// // You can reinitialize it here if needed
    pub fn mem_init(){
        let size_memory = mem_space_get_size();
        let first_memory_block = mem_space_get_addr() as *mut MemFreeBlock;

        unsafe {
            (*first_memory_block).next = None;
            (*first_memory_block).size = size_memory;
        }

        Self::mem_set_fit_handler(Self::mem_first_fit);
    }

    //-------------------------------------------------------------
    // Iterator (parcours) over the content of the allocator
    // mem_show
    //-------------------------------------------------------------
    pub fn mem_show(print: fn(*mut u8, usize, bool)) {
        let ptr_memory = mem_space_get_addr() as *mut u8;
        let mut ptr_current = ptr_memory;
        let size_mem = mem_space_get_size();
        let end_memory = unsafe { ptr_memory.add(size_mem) };

        let mut free_b = unsafe { &*(ptr_memory as *mut MemFreeBlock) };

        while ptr_current != end_memory {
            if ptr_current == (free_b as *const MemFreeBlock as *mut u8) {
                print(ptr_current, free_b.size, true);
                ptr_current = unsafe { ptr_current.add(free_b.size) };
                free_b = match &free_b.next {
                    Some(next_block) => next_block,
                    None => break,
                };
            } else {
                let busy_zone = unsafe { &*(ptr_current as *mut MemMetaBlock) };
                print(ptr_current, busy_zone.size, false);
                ptr_current = unsafe { ptr_current.add(busy_zone.size) };
            }
        }
    }

    // Get a reference to the next block
    pub fn get_next(&self) -> Option<&Box<MemFreeBlock>> {
        self.next.as_ref() // Return a reference to the next block
    }

    // Get the size of the block
    pub fn get_size(&self) -> usize {
        self.size
    }

    // Set the next block
    pub fn set_next(&mut self, n: Option<Box<MemFreeBlock>>) {
        self.next = n;
    }

    // Set the size of the block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }
    //-------------------------------------------------------------
    // mem_fit
    //-------------------------------------------------------------
    pub fn mem_set_fit_handler(mff: fn(*mut MemFreeBlock, usize) -> *mut MemFreeBlock) {
        // TODO: implement
        unsafe {
            FIT_HANDLER = Some(mff);
        }
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
}

pub struct MemMetaBlock {
    pub size: usize, // Size of the block
}

impl MemMetaBlock {
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
    // Get the size of the block
    pub fn get_size(&self) -> usize {
        self.size
    }
    // Set the size of the block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }
}
