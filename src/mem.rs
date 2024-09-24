use std::{assert, ptr};
use crate::mem_space::*;

// Global static variables
static mut FIT_HANDLER: Option<fn(&mut MemFreeBlock, usize) -> Option<&mut MemFreeBlock>> = None;
static mut FREE_LIST_HEAD: Option<Box<MemFreeBlock>> = None; // Head of the free list

// Free block structure
pub struct MemFreeBlock {
    pub next: Option<Box<MemFreeBlock>>, // Pointer to the next free block (linked list)
    pub size: usize,                     // Size of the free block
}

impl MemFreeBlock {
    // Get the first free block
    fn get_first_block() -> Option<&'static mut MemFreeBlock> {
        unsafe {
            FREE_LIST_HEAD.as_deref_mut() // Use the head pointer and return an immutable reference
        }
    }

    pub fn mem_init() {
        let size_memory = mem_space_get_size();
        let first_memory_block_ptr = mem_space_get_addr() as *mut MemFreeBlock; // Get a raw pointer to MemFreeBlock
        
        // Safely initialize the first memory block
        unsafe {
            // Create a new free block at the start of the memory space
            (*first_memory_block_ptr).set_next(None);
            (*first_memory_block_ptr).set_size(size_memory);
    
            // Convert the raw pointer into a Box and assign it to the head of the free list
            FREE_LIST_HEAD = Some(Box::from_raw(first_memory_block_ptr));
        }
    
        // Set the default memory fit handler (e.g., first fit)
        Self::mem_set_fit_handler(Self::mem_first_fit);
    }

    // Get the next block (returns a mutable reference)
    pub fn get_next(&mut self) -> Option<&mut MemFreeBlock> {
        self.next.as_deref_mut() // Convert Option<Box<T>> to Option<&mut T>
    }

    // Get the size of the block
    pub fn get_size(&self) -> usize {
        self.size
    }

    // Set the next block
    pub fn set_next(&mut self, n: Option<MemFreeBlock>) {
        self.next = n.map(Box::new); // Safely handle Option and Box the new block
    }

    // Set the size of the block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }

    // Delete a block from the free list
    pub fn delete(target_block: &mut MemFreeBlock) {
        let mut current_block = Self::get_first_block();

        if current_block.is_none() {
            return; // Early exit if the free list is empty
        }

        let mut prev_block: Option<&mut MemFreeBlock> = None;

        // Traverse the free list to find the target block
        while let Some(current) = current_block {
            if ptr::eq(current, target_block) {
                if let Some(prev) = prev_block {
                    // Set the previous block's next pointer to skip the current block
                    prev.next = current.next.take();
                } else {
                    // Handle case where the block to delete is the head
                    unsafe {
                        FREE_LIST_HEAD = current.next.take();
                    }
                }
                return; // Block deleted
            }

            prev_block = Some(unsafe { &mut *(current as *const _ as *mut MemFreeBlock) }); // Safe casting to mutable reference
            current_block = current.get_next();
        }
    }

    // Replace the target block in the free list with a new block
    pub fn replace(target_block: &mut MemFreeBlock, new_block_ptr: *mut MemFreeBlock) {
        let mut current_block = unsafe { FREE_LIST_HEAD.as_deref_mut() };

        if current_block.is_none() {
            return; // Early exit if the free list is empty
        }

        let mut prev_block: Option<&mut MemFreeBlock> = None;

        // Traverse the free list to find the target block
        while let Some(current) = current_block {
            if ptr::eq(current, target_block) {
                let mut new_block_box = unsafe { Box::from_raw(new_block_ptr) }; // Wrap new block in a Box
                new_block_box.next = current.next.take();    // Link new block to current block's next

                if let Some(prev) = prev_block {
                    prev.next = Some(new_block_box); // Set the previous block's next to the new block
                } else {
                    unsafe {
                        FREE_LIST_HEAD = Some(new_block_box); // Replace head if target is the first block
                    }
                }
                return; // Target block replaced
            }

            prev_block = Some(unsafe { &mut *(current as *const _ as *mut MemFreeBlock) }); // Safe casting to mutable reference
            current_block = current.get_next();
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
                if let Some(next_block) = free_block.next.as_deref_mut() {
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
    // Allocate memory
    pub fn mem_alloc(size: usize) -> *mut u8 {
        // Check if there is a fit handler set
        if let Some(handler) = unsafe { FIT_HANDLER } {
            // Get the first free block
            if let Some(first_free_block) = MemFreeBlock::get_first_block() {
                // Use the fit strategy to find a suitable block
                if let Some(suitable_block) = handler(first_free_block, size + std::mem::size_of::<MemMetaBlock>()) {
                    // Save the pointer to the suitable block
                    let suitable_block_ptr = suitable_block as *mut MemFreeBlock;

                    // Calculate the total size needed for the allocation (including metadata)
                    let total_alloc_size = size + std::mem::size_of::<MemMetaBlock>();

                    // Get the size of the suitable free block
                    let suitable_block_size = suitable_block.get_size();

                    // Calculate the leftover size after allocation
                    let leftover_size = suitable_block_size - total_alloc_size;

                    // If there's enough space left to create a new free block after the allocation
                    if leftover_size >= std::mem::size_of::<MemFreeBlock>() {
                        // Pointer arithmetic to calculate the address of the new free block
                        let new_free_block_ptr = unsafe {
                            (suitable_block_ptr as *mut u8).add(total_alloc_size) as *mut MemFreeBlock
                        };

                        // Initialize the new free block
                        unsafe {
                            (*new_free_block_ptr).set_size(leftover_size);
                            (*new_free_block_ptr).next = suitable_block.next.take();
                        }

                        // Replace the suitable block with the new free block in the free list
                        MemFreeBlock::replace(suitable_block,new_free_block_ptr);
                    } else {
                        // If no split is needed (remaining block is too small), just remove the block
                        MemFreeBlock::delete(suitable_block);
                    }

                    // Set the size of the allocated block (excluding the metadata size)
                    unsafe {
                        (*suitable_block_ptr).set_size(size);
                    }

                    // Return the pointer to the allocated memory (just after the metadata)
                    return unsafe { (suitable_block_ptr as *mut u8).add(std::mem::size_of::<MemMetaBlock>()) };
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
