use std::{assert, ptr};
use crate::mem_space::*;

static mut FIT_HANDLER: Option<fn(&MemFreeBlock, usize) -> Option<&MemFreeBlock>> = None;

pub struct MemFreeBlock {
    pub next: Option<Box<MemFreeBlock>>, // Pointer to the next free block (linked list)
    pub size: usize,                     // Size of the free block
}

impl MemFreeBlock {
    fn get_first_block() -> Option<&'static MemFreeBlock> {
        let first_memory_block = mem_space_get_addr() as *mut MemFreeBlock;
        unsafe { (first_memory_block as *const MemFreeBlock).as_ref() }
    }

    //-------------------------------------------------------------
    // mem_init -- constructor
    //-------------------------------------------------------------
    pub fn mem_init() {
        let size_memory = mem_space_get_size();
        let first_memory_block = mem_space_get_addr() as *mut MemFreeBlock;
        let first_memory_block = unsafe { &mut *(first_memory_block as *mut MemFreeBlock) };

        first_memory_block.next = None;
        first_memory_block.size = size_memory;

        //Self::mem_set_fit_handler(Self::mem_first_fit);
    }

    // Get the next block
    pub fn get_next(&self) -> Option<&MemFreeBlock> {
        self.next.as_ref().map(|free_block| free_block.as_ref())
    }

    // Get the size of the block
    pub fn get_size(&self) -> usize {
        self.size
    }

    // Set the next block
    pub fn set_next(&mut self, n: MemFreeBlock) {
        self.next = Some(Box::from(n));
    }

    // Set the size of the block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }

    //-------------------------------------------------------------
    // mem_show -- show free and occupied memory
    //-------------------------------------------------------------
    pub fn mem_show(print: fn(*mut u8, usize, bool)) {
        let ptr_memory = mem_space_get_addr() as *mut u8;
        let mut ptr_current = ptr_memory;
        let size_mem = mem_space_get_size();
        let end_memory = unsafe { ptr_memory.add(size_mem) };
        let ffb = Self::get_first_block();
        let mut free_block = ffb.unwrap();

            while ptr_current != end_memory {
                    // case current block is free
                    if ptr_current == (free_block as *const MemFreeBlock as *mut u8) {
                        print(ptr_current, free_block.get_size(), true);
                        ptr_current = ptr_current.wrapping_add(free_block.size);
                        let next_block = free_block.get_next();
                        if !next_block.is_none() {
                            free_block = next_block.unwrap();
                        } else {
                            break;
                        }
                    // case current block is used
                    } else {
                        let busy_zone = unsafe { &*(ptr_current as *mut MemMetaBlock) };
                        print(ptr_current, busy_zone.size, false);
                        ptr_current = ptr_current.wrapping_add(busy_zone.size);
                    }
                }
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
        let mut fb: &MemFreeBlock = first_free_block;
    
        // Iterate through the free blocks
        loop {
            // Check if the current free block is large enough
            if fb.get_size() >= wanted_size + std::mem::size_of::<MemMetaBlock>() {
                return Some(fb);
            }
            
            // Move to the next block
            match fb.get_next() {
                None => return None, // No suitable block found
                Some(next_block) => fb = next_block, // Move to the next block
            }
        }
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
    pub fn mem_alloc(size: usize) -> *mut u8 {
        unsafe {
            if let Some(handler) = FIT_HANDLER {
                let first_free_block = MemFreeBlock::get_first_block();
                if let Some(free_block) = first_free_block {
                    if let Some(allocated_block) = handler(free_block, size) {
                        let total_allocated_size = size + std::mem::size_of::<MemMetaBlock>();
                        
                        // Create a new metadata block for the allocated memory
                        let _meta_block = MemMetaBlock {
                            size: total_allocated_size,
                        };
    
                        // Update the free block after allocation
                        //MemFreeBlock::update_mem_free_after_alloc(free_block, size);
    
                        // Return a pointer to the allocated memory after the metadata
                        return (allocated_block as *const MemFreeBlock as *mut u8).wrapping_add(std::mem::size_of::<MemMetaBlock>());
                    }
                }
            }
        }
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
