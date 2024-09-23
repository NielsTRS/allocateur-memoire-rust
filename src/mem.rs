use std::{assert, ptr};
use crate::mem_space::*;

static mut FIT_HANDLER: Option<fn(&mut MemFreeBlock, usize) -> Option<&mut MemFreeBlock>> = None;

pub struct MemFreeBlock {
    pub next: Option<Box<MemFreeBlock>>, // Pointer to the next free block (linked list)
    pub size: usize,                     // Size of the free block
}

impl MemFreeBlock {
    fn get_first_block() -> Option<&'static mut MemFreeBlock> {
        let first_memory_block = mem_space_get_addr() as *mut MemFreeBlock;
        unsafe { first_memory_block.as_mut() }
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
    pub fn get_next(&mut self) -> Option<&mut MemFreeBlock> {
        self.next.as_deref_mut()
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

    pub fn delete(target_block: &mut MemFreeBlock) {
        let first_block = Self::get_first_block();

        // Early exit if the free list is empty
        if first_block.is_none() {
            return;
        }

        let mut current_block = first_block.unwrap();
        let mut prev_block: Option<&mut MemFreeBlock> = None;

        // Traverse the free list to find the target block
        loop {
            // Check if the current block is the target block
            if ptr::eq(current_block, target_block) {
                if let Some(prev) = prev_block {
                    // If the current block has a next block, set the previous block's next pointer to it
                    prev.next = current_block.next.take();
                } else {
                    // Handle the case where the block to delete is the first block in the list
                    let first_memory_block = mem_space_get_addr() as *mut MemFreeBlock;
                    if let Some(next_block) = current_block.next.take() {
                        unsafe {
                            *first_memory_block = *next_block;
                        }
                    } else {
                        // If there's no next block, the free list becomes empty
                        unsafe {
                            *first_memory_block = MemFreeBlock {
                                next: None,
                                size: 0,
                            };
                        }
                    }
                }
                return; // Target block found and deleted, exit the function
            }

            // Move to the next block in the list
            prev_block = Some(unsafe { &mut *(current_block as *const _ as *mut MemFreeBlock) });
            current_block = current_block.get_next().unwrap();
        }
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
                        let next_block = free_block.next.as_mut();
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
    pub fn mem_set_fit_handler(mff: fn(&mut MemFreeBlock, usize) -> Option<&mut MemFreeBlock>) {
        unsafe {
            FIT_HANDLER = Some(mff);
        }
    }

    //-------------------------------------------------------------
    // First Fit Strategy
    //-------------------------------------------------------------
    pub fn mem_first_fit(first_free_block: &mut MemFreeBlock, wanted_size: usize) -> Option<&mut MemFreeBlock> {
        let mut fb = first_free_block;
    
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
        // Check if there is a fit handler set
        if let Some(handler) = unsafe { FIT_HANDLER } {
            // Get the first free block
            if let Some(free_block) = MemFreeBlock::get_first_block() {
                // Use the fit strategy to find a suitable block
                if let Some(suitable_block) = handler(free_block, size + std::mem::size_of::<MemMetaBlock>()) {
                    // Save the pointer to the suitable block before deleting it
                    let suitable_block_ptr = suitable_block as *const MemFreeBlock as *mut MemFreeBlock;

                    // Delete the block from the free list
                    MemFreeBlock::delete(suitable_block);

                    // Calculate the total size needed for the allocation (including meta)
                    let total_alloc_size = size + std::mem::size_of::<MemMetaBlock>();

                    // Check if we need to split the free block
                    unsafe {
                        if (*suitable_block_ptr).get_size() > total_alloc_size {
                            // Create a new free block from the remaining memory after allocation
                            let new_free_block_ptr = (suitable_block_ptr as *mut u8).add(total_alloc_size) as *mut MemFreeBlock;
                            let new_free_block = &mut *new_free_block_ptr;

                            // Update the new free block
                            new_free_block.set_size((*suitable_block_ptr).get_size() - total_alloc_size);
                            if let Some(next_block) = Some(*new_free_block.next.take().unwrap()) {
                                new_free_block.set_next(next_block);
                            } else {
                                new_free_block.next = None;
                            }

                            // Add the new free block to the free list
                            let first_block = MemFreeBlock::get_first_block();
                            if let Some(first_free) = first_block {
                                new_free_block.set_next(*first_free.next.take().unwrap());
                            }
                        }

                        // Set the allocated block size (without splitting) in the meta block
                        (*suitable_block_ptr).set_size(size);
                    }

                    // Return the pointer to the allocated memory (just after the meta block)
                    return unsafe { (suitable_block_ptr as *mut u8).add(std::mem::size_of::<MemMetaBlock>()) };
                }
            }
        }

        // Return null if no suitable block was found
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
