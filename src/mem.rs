use std::{assert, ptr};
use crate::mem_space::*;

static mut FIT_HANDLER: Option<fn(&MemFreeBlock, usize) -> Option<&MemFreeBlock>> = None;

pub struct MemFreeBlock {
    pub next: Option<Box<MemFreeBlock>>, // Pointer to the next free block (linked list)
    pub size: usize,                     // Size of the free block
}

impl MemFreeBlock {

    //-------------------------------------------------------------
    // mem_init
    //-------------------------------------------------------------
    pub fn mem_init() {
        let size_memory = mem_space_get_size();
        let first_memory_block = mem_space_get_addr() as *mut MemFreeBlock;

        let first_memory_block = unsafe { &mut *(first_memory_block as *mut MemFreeBlock) };

        first_memory_block.next = None;
        first_memory_block.size = size_memory;

        Self::mem_set_fit_handler(Self::mem_first_fit);
    }

    //-------------------------------------------------------------
    // mem_show
    //-------------------------------------------------------------
    pub fn mem_show(print: fn(*mut u8, usize, bool)) {
        let ptr_memory = mem_space_get_addr() as *mut u8;
        let mut ptr_current = ptr_memory;
        let size_mem = mem_space_get_size();
        let end_memory = unsafe { ptr_memory.add(size_mem) };

        let mut free_b = Self::get_first_block();

        while ptr_current != end_memory {
            if let Some(free_block) = free_b {
                if ptr_current == (free_block as *const MemFreeBlock as *mut u8) {
                    print(ptr_current, free_block.get_size(), true);
                    ptr_current = ptr_current.wrapping_add(free_block.size);
                    free_b = free_block.next.as_deref_mut(); // Safe dereferencing
                } else {
                    let busy_zone = unsafe { &*(ptr_current as *mut MemMetaBlock) };
                    print(ptr_current, busy_zone.size, false);
                    ptr_current = ptr_current.wrapping_add(busy_zone.size);
                }
            }
        }
    }

    //-------------------------------------------------------------
    // Get the first block
    //-------------------------------------------------------------
    fn get_first_block() -> Option<&'static mut MemFreeBlock> {
        let first_memory_block = mem_space_get_addr() as *mut MemFreeBlock;
        unsafe { (first_memory_block as *mut MemFreeBlock).as_mut() }
    }

    // Get the next block
    pub fn get_next(&self) -> Option<&MemFreeBlock> {
        self.next.as_deref()
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
    pub fn mem_set_fit_handler(mff: fn(&MemFreeBlock, usize) -> Option<&MemFreeBlock>) {
        unsafe {
            FIT_HANDLER = Some(mff);
        }
    }

    //-------------------------------------------------------------
    // First Fit Strategy
    //-------------------------------------------------------------
    pub fn mem_first_fit(first_free_block: &MemFreeBlock, wanted_size: usize) -> Option<&MemFreeBlock> {
        let mut fb = first_free_block;

        while fb.get_size() < wanted_size + 8 {
            match fb.get_next() {
                None => return None, // No suitable block found
                Some(next_block) => fb = next_block, // Move to the next block
            }
        }

        Some(fb)
    }

    //-------------------------------------------------------------
    // Best Fit Strategy (Still TODO)
    //-------------------------------------------------------------
    pub fn mem_best_fit(_first_free_block: &mut MemFreeBlock, _wanted_size: usize) -> Option<&mut MemFreeBlock> {
        assert!(false, "NOT IMPLEMENTED !");
        None
    }

    //-------------------------------------------------------------
    // Worst Fit Strategy (Still TODO)
    //-------------------------------------------------------------
    pub fn mem_worst_fit(_first_free_block: &mut MemFreeBlock, _wanted_size: usize) -> Option<&mut MemFreeBlock> {
        assert!(false, "NOT IMPLEMENTED !");
        None
    }
}

pub struct MemMetaBlock {
    pub size: usize, // Size of the block
}

impl MemMetaBlock {
    //-------------------------------------------------------------
    // mem_alloc
    //-------------------------------------------------------------
    pub fn mem_alloc(_size: usize) -> *mut u8 {
        assert!(false, "NOT IMPLEMENTED !");
        ptr::null_mut()
    }

    //-------------------------------------------------------------
    // mem_get_size
    //-------------------------------------------------------------
    pub fn mem_get_size(_zone: *mut u8) -> usize {
        assert!(false, "NOT IMPLEMENTED !");
        0
    }

    //-------------------------------------------------------------
    // mem_free
    //-------------------------------------------------------------
    pub fn mem_free(_zone: *mut u8) {
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
