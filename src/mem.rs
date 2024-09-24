use std::{assert, ptr};
use crate::mem_space::*;

// Global static variables
static mut FIT_HANDLER: Option<fn(&mut MemFreeBlock, usize) -> Option<&mut MemFreeBlock>> = None;
static mut FIRST_FREE_BLOCK: *mut MemFreeBlock = ptr::null_mut(); // Head of the free list

// Free block structure
pub struct MemFreeBlock {
    pub next: *mut MemFreeBlock, // Pointer to the next free block (linked list)
    pub size: usize,             // Size of the free block
}

impl MemFreeBlock {
    // Get the first free block
    fn get_first_block() -> Option<&'static mut MemFreeBlock> {
        unsafe {
            if FIRST_FREE_BLOCK.is_null() {
                None
            } else {
                Some(&mut *FIRST_FREE_BLOCK) // Dereference the raw pointer
            }
        }
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn mem_init() {
        let size_memory = mem_space_get_size();
        let first_memory_block_ptr = mem_space_get_addr() as *mut MemFreeBlock; // Get a raw pointer to MemFreeBlock

        // Safely initialize the first memory block
        unsafe {
            // Initialize the first free block in place
            (*first_memory_block_ptr).next = ptr::null_mut(); // Set next to null
            (*first_memory_block_ptr).set_size(size_memory);

            // Assign the head of the free list
            FIRST_FREE_BLOCK = first_memory_block_ptr;
        }

        // Set the default memory fit handler (e.g., first fit)
        Self::mem_set_fit_handler(Self::mem_first_fit);
    }

    // Get the next block (returns a mutable reference)
    pub fn get_next(&mut self) -> Option<&mut MemFreeBlock> {
        if self.next.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.next) } // Safely dereference
        }
    }

    // Get the size of the block
    pub fn get_size(&self) -> usize {
        self.size
    }

    // Delete a block from the free list
pub fn delete(target_block: &mut MemFreeBlock) {
    unsafe {
        // Get the first block
        let mut current_block = FIRST_FREE_BLOCK;
        let mut prev_block: *mut MemFreeBlock = ptr::null_mut();

        // Traverse the list to find the target_block
        while !current_block.is_null() {
            if current_block == target_block as *mut MemFreeBlock {
                // If the block to delete is the first block
                if prev_block.is_null() {
                    FIRST_FREE_BLOCK = (*current_block).next; // Update head
                } else {
                    (*prev_block).next = (*current_block).next; // Bypass the target_block
                }
                return; // Block deleted
            }
            prev_block = current_block; // Move to next block
            current_block = (*current_block).next; // Move to the next block
        }
    }
}

// Replace the target block in the free list with a new block
pub fn replace(target_block: &mut MemFreeBlock, new_block: &mut MemFreeBlock) {
    unsafe {
        let mut current_block = FIRST_FREE_BLOCK;

        // Traverse the list to find the target_block
        while !current_block.is_null() {
            if current_block == target_block as *mut MemFreeBlock {
                // Replace the target block with the new block
                new_block.next = (*current_block).next; // Link new block to next block
                // If replacing the first block
                if current_block == FIRST_FREE_BLOCK {
                    FIRST_FREE_BLOCK = new_block; // Update head to new block
                } else {
                    // Find previous block
                    let mut prev_block: *mut MemFreeBlock = ptr::null_mut();
                    let mut temp_block = FIRST_FREE_BLOCK;
                    while temp_block != current_block {
                        prev_block = temp_block;
                        temp_block = (*temp_block).next;
                    }
                    (*prev_block).next = new_block; // Link previous block to new block
                }
                return; // Block replaced
            }
            current_block = (*current_block).next; // Move to the next block
        }
    }
}


    // Show the free and occupied memory blocks
    pub fn mem_show(print: fn(*mut u8, usize, bool)) {
        let ptr_memory = mem_space_get_addr() as *mut u8;
        let mut ptr_current = ptr_memory;
        let size_mem = mem_space_get_size();
        let end_memory = unsafe { ptr_memory.add(size_mem) };
        let mut free_block = Self::get_first_block().unwrap();

        while ptr_current != end_memory {
            // If current block is free
            if ptr_current == (free_block as *const MemFreeBlock as *mut u8) {
                print(ptr_current, free_block.get_size(), true);
                ptr_current = ptr_current.wrapping_add(free_block.get_size());
                if let Some(next_block) = free_block.get_next() {
                    free_block = next_block;
                } else {
                    break;
                }
            } else {
                let busy_zone = unsafe { &*(ptr_current as *mut MemMetaBlock) };
                print(ptr_current, busy_zone.size, false);
                ptr_current = ptr_current.wrapping_add(busy_zone.size);
            }
        }
    }

    // Set the memory fit handler strategy
    pub fn mem_set_fit_handler(mff: fn(&mut MemFreeBlock, usize) -> Option<&mut MemFreeBlock>) {
        unsafe {
            FIT_HANDLER = Some(mff);
        }
    }

    // First Fit Strategy
    pub fn mem_first_fit(first_free_block: &mut MemFreeBlock, wanted_size: usize) -> Option<&mut MemFreeBlock> {
        let mut fb = first_free_block;

        loop {
            if fb.get_size() >= wanted_size + std::mem::size_of::<MemMetaBlock>() {
                return Some(fb);
            }

            match fb.get_next() {
                None => return None,
                Some(next_block) => fb = next_block,
            }
        }
    }

    // Best Fit Strategy (Not implemented)
    pub fn mem_best_fit(_first_free_block: &mut MemFreeBlock, _wanted_size: usize) -> Option<&mut MemFreeBlock> {
        assert!(false, "NOT IMPLEMENTED !");
        None
    }

    // Worst Fit Strategy (Not implemented)
    pub fn mem_worst_fit(_first_free_block: &mut MemFreeBlock, _wanted_size: usize) -> Option<&mut MemFreeBlock> {
        assert!(false, "NOT IMPLEMENTED !");
        None
    }
}

pub struct MemMetaBlock {
    pub size: usize,
}

impl MemMetaBlock {
    pub fn mem_alloc(size: usize) -> *mut u8 {
        // Check if there is a fit handler set
        if let Some(handler) = unsafe { FIT_HANDLER } {
            // Get the first free block
            if let Some(first_free_block) = MemFreeBlock::get_first_block() {
                // Use the fit strategy to find a suitable block
                if let Some(suitable_block) = handler(first_free_block, size + std::mem::size_of::<MemMetaBlock>()) {
                    let suitable_block_ptr = suitable_block as *mut MemFreeBlock;
    
                    // Calculate the total size needed for the allocation (including metadata)
                    let total_alloc_size = size + std::mem::size_of::<MemMetaBlock>();
    
                    // Get the size of the suitable free block
                    let suitable_block_size = suitable_block.get_size();
    
                    // Calculate the leftover size after allocation
                    let leftover_size = suitable_block_size - total_alloc_size;
    
                    if leftover_size >= std::mem::size_of::<MemFreeBlock>() {
                        // Pointer arithmetic to calculate the address of the new free block
                        let new_free_block_ptr = unsafe {
                            (suitable_block_ptr as *mut u8).add(total_alloc_size) as *mut MemFreeBlock
                        };
    
                        // Initialize the new free block
                        let mut new_free_block = unsafe {
                            std::ptr::read(new_free_block_ptr)
                        };
                        new_free_block.set_size(leftover_size);
    
                        // Replace the suitable block with the new free block in the free list
                        MemFreeBlock::replace(suitable_block, &mut new_free_block);
                    } else {
                        // If no split is needed (remaining block is too small), just remove the block
                        MemFreeBlock::delete(suitable_block);
                    }
    
                    // Create and set the size of the allocated MemMetaBlock
                    let meta_block_ptr = (suitable_block_ptr as *mut u8) as *mut MemMetaBlock;
    
                    unsafe {
                        (*meta_block_ptr).size = size; // Set the size in the metadata block
                    }
    
                    // Return the pointer to the allocated memory (just after the metadata)
                    return unsafe { (meta_block_ptr as *mut u8).add(std::mem::size_of::<MemMetaBlock>()) };
                }
            }
        }
    
        // Return null if no suitable block was found
        ptr::null_mut()
    }
    

    // Get the size of a block (not implemented)
    pub fn mem_get_size(_zone: *mut u8) -> usize {
        assert!(false, "NOT IMPLEMENTED !");
        0
    }

    // Free memory (not implemented)
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
