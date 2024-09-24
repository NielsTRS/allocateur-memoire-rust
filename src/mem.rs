use std::{assert, ptr};
use crate::mem_space::*;

// Global static variables
type FitHandler = fn(*mut MemFreeBlock, usize) -> Option<*mut MemFreeBlock>;
static mut FIT_HANDLER: Option<FitHandler> = None;

static mut FREE_LIST_HEAD: Option<*mut MemFreeBlock> = None;

pub struct MemMetaBlock {
    pub size: usize,
}

pub struct MemFreeBlock {
    pub size: usize,
    pub next: Option<*mut MemFreeBlock>, // Pointer to the next free block
}

impl MemFreeBlock {
    // Get the first free block in the list.
    pub fn get_first_block() -> Option<*mut MemFreeBlock> {
        unsafe {
            FREE_LIST_HEAD // Return the head of the free list
        }
    }

    // Set the first free block (used to initialize or modify the list).
    pub fn set_first_block(block: *mut MemFreeBlock) {
        unsafe {
            FREE_LIST_HEAD = Some(block);
        }
    }

    pub fn mem_init() {
        let size_memory = mem_space_get_size();
        let first_memory_block_ptr = mem_space_get_addr() as *mut MemFreeBlock; // Get a raw pointer to MemFreeBlock
        
        // Safely initialize the first memory block
        unsafe {
            // Create a new free block at the start of the memory space
            (*first_memory_block_ptr).next = None;
            (*first_memory_block_ptr).set_size(size_memory);
    
            MemFreeBlock::set_first_block(first_memory_block_ptr);
        }
    
        // Set the default memory fit handler (e.g., first fit)
        Self::mem_set_fit_handler(Self::mem_first_fit);
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_next(&self) -> Option<*mut MemFreeBlock> {
        self.next
    }

    // Set the size of the block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }

    // Show the free and occupied memory blocks
    pub fn mem_show(print: fn(*mut u8, usize, bool)) {
        // Get the starting address and size of the memory space
        let ptr_memory = mem_space_get_addr() as *mut u8;
        let mut ptr_current = ptr_memory;  // Pointer to the current memory block
        let size_mem = mem_space_get_size();  // Total size of the memory space
        let end_memory = unsafe { ptr_memory.add(size_mem) };  // End of the memory space
    
        // Get the first free block
        let mut free_block = MemFreeBlock::get_first_block();  // Get the first free block in the free list
    
        // Iterate through the entire memory space
        while ptr_current < end_memory {
            if let Some(free_block_ptr) = free_block {
                // If the current block is a free block
                if ptr_current == free_block_ptr as *mut u8 {
                    unsafe {
                        // Print free block information: address, size, and mark it as free
                        print(ptr_current, (*free_block_ptr).get_size(), true);
    
                        // Move the current pointer forward by the size of the free block
                        ptr_current = ptr_current.add((*free_block_ptr).get_size());
    
                        // Move to the next free block in the list
                        free_block = (*free_block_ptr).get_next();
                    }
                } else {
                    // If the current block is allocated (not free)
                    let busy_zone = unsafe { &*(ptr_current as *mut MemMetaBlock) };
    
                    // Print busy block information: address, size, and mark it as allocated
                    print(ptr_current, busy_zone.size, false);
    
                    // Move the current pointer forward by the size of the busy block
                    ptr_current = unsafe {
                        ptr_current.add(busy_zone.size)
                    };
                }
            } else {
                // If there are no more free blocks, treat the rest as allocated
                let busy_zone = unsafe { &*(ptr_current as *mut MemMetaBlock) };
    
                // Print busy block information
                print(ptr_current, busy_zone.size, false);
    
                // Move the pointer forward
                ptr_current = unsafe {
                    ptr_current.add(busy_zone.size)
                };
            }
        }
    }

    /// Replaces an old free block with a new free block in the free list.
    pub fn replace(old_block: *mut MemFreeBlock, new_block: *mut MemFreeBlock) {
        unsafe {
            if old_block == new_block {
                return;  // If the blocks are the same, no need to replace
            }

            // Check if the old block is the first in the list
            if let Some(mut first_block) = MemFreeBlock::get_first_block() {
                if first_block == old_block {
                    // If the old block is the first block, replace it with the new one
                    MemFreeBlock::set_first_block(new_block);
                    (*new_block).next = (*old_block).next;
                    return;
                }

                // Traverse the list to find the old block and replace it
                let mut current_block = first_block;
                while let Some(next_block) = (*current_block).next {
                    if next_block == old_block {
                        // Replace the old block with the new one
                        (*current_block).next = Some(new_block);
                        (*new_block).next = (*old_block).next;
                        return;
                    }
                    current_block = next_block;
                }
            }
        }
    }

    /// Deletes a free block from the free list.
    pub fn delete(block_to_delete: *mut MemFreeBlock) {
        unsafe {
            // Check if the block to delete is the first block in the list
            if let Some(mut first_block) = MemFreeBlock::get_first_block() {
                if first_block == block_to_delete {
                    // If the block to delete is the first block, update the head of the list
                    MemFreeBlock::set_first_block((*block_to_delete).next.unwrap_or(ptr::null_mut()));
                    return;
                }

                // Traverse the list to find the block and remove it
                let mut current_block = first_block;
                while let Some(next_block) = (*current_block).next {
                    if next_block == block_to_delete {
                        // Remove the block by bypassing it in the linked list
                        (*current_block).next = (*block_to_delete).next;
                        return;
                    }
                    current_block = next_block;
                }
            }
        }
    }
    

    // Set the memory fit handler strategy
    pub fn mem_set_fit_handler(handler: FitHandler) {
        unsafe {
            FIT_HANDLER = Some(handler);
        }
    }

    // First Fit Strategy
    fn mem_first_fit(first_block: *mut MemFreeBlock, size: usize) -> Option<*mut MemFreeBlock> {
        let mut current_block = first_block;
        
        // Iterate over free blocks and find the first suitable block
        while !current_block.is_null() {
            unsafe {
                let block = &*current_block;
                if block.get_size() >= size {
                    return Some(current_block);
                }
                current_block = block.get_next().unwrap_or(ptr::null_mut());
            }
        }
    
        // No suitable block found
        None
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

impl MemMetaBlock {
    // Allocate memory
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
                    let suitable_block_size = unsafe { (*suitable_block_ptr).get_size() };
    
                    // Calculate the leftover size after allocation
                    let leftover_size = suitable_block_size - total_alloc_size;
    
                    if leftover_size >= std::mem::size_of::<MemFreeBlock>() {
                        // Pointer arithmetic to calculate the address of the new free block
                        let new_free_block_ptr = unsafe {
                            (suitable_block_ptr as *mut u8).add(total_alloc_size) as *mut MemFreeBlock
                        };
    
                        // Initialize the new free block
                        unsafe {
                            (*new_free_block_ptr).set_size(leftover_size);
                        }
    
                        // Replace the suitable block with the new free block in the free list
                        MemFreeBlock::replace(suitable_block, new_free_block_ptr );
                    } else {
                        // If no split is needed (remaining block is too small), just remove the block
                        MemFreeBlock::delete(suitable_block);
                    }
    
                    // Create and set the size of the allocated MemMetaBlock
                    let meta_block_ptr = (suitable_block_ptr as *mut u8) as *mut MemMetaBlock;
    
                    unsafe {
                        (*meta_block_ptr).size = size+8; // Set the size in the metadata block
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
