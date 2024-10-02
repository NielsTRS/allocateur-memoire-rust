use crate::mem_space::*;
use std::ptr;

// Global static variables
type FitHandler = fn(*mut MemFreeBlock, usize) -> Option<*mut MemFreeBlock>;
static mut FIT_HANDLER: Option<FitHandler> = None;
pub static MODULO: usize = 8;

static mut FREE_LIST_HEAD: Option<*mut MemFreeBlock> = None;

pub struct MemMetaBlock {
    pub size: usize,
}

// Linked list for free memory management
pub struct MemFreeBlock {
    pub size: usize,
    pub next: Option<*mut MemFreeBlock>,
}

// Get the next modulo 8 size
pub fn get_modulo(size: usize) -> usize {
    if size % MODULO != 0 {
        size + (MODULO - (size % MODULO))
    } else {
        size
    }
}

impl MemFreeBlock {
    // Get the list head
    pub fn get_first_block() -> Option<*mut MemFreeBlock> {
        unsafe { FREE_LIST_HEAD }
    }
    // Set the head of the list
    pub fn set_first_block(block: *mut MemFreeBlock) {
        unsafe {
            FREE_LIST_HEAD = Some(block);
        }
    }

    pub fn mem_init() {
        // Get the starting address and size of the memory space
        let mem_start = mem_space_get_addr();
        let mem_size = mem_space_get_size();
    
        // Reserve space for the free list at the beginning of the memory
        let free_list_size = std::mem::size_of::<MemFreeBlock>();

        // Calculate the size of the first free block
        let first_free_block_size = mem_size - free_list_size;
        let first_free_block_ptr = unsafe { (mem_start as *mut u8).add(free_list_size) as *mut MemFreeBlock };
    
        // Initialize the first free block
        unsafe {
            (*first_free_block_ptr).size = first_free_block_size;
            (*first_free_block_ptr).next = None;
    
            // Set the first block in the free list
            MemFreeBlock::set_first_block(first_free_block_ptr);
        }
    
        // Set the default memory fit handler (first_fit)
        Self::mem_set_fit_handler(Self::mem_first_fit);
    }

    // get current block size
    fn get_size(&self) -> usize {
        self.size
    }
    // get current block next (None or MemFreeBlock)
    fn get_next(&self) -> Option<*mut MemFreeBlock> {
        self.next
    }
    // set the size of the current block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }

    // Show the free and occupied memory blocks
    pub fn mem_show(print: fn(usize, usize, bool)) {
        Self::print_free_list();

        let mem_ptr = mem_space_get_addr();
    
        // Get the starting address and size of the memory space
        let mem_start = unsafe { mem_space_get_addr().add(std::mem::size_of::<MemFreeBlock>()) };
        let mem_size = mem_space_get_size();
    
        let mut ptr_current = mem_start; // Pointer to the current memory block, starting after the free list
        let end_memory = unsafe { mem_ptr.add(mem_size) }; // End of the memory space address
    
        // Get the first free block
        let mut free_block = MemFreeBlock::get_first_block();
    
        // Iterate through the entire memory space
        while ptr_current < end_memory {
            if let Some(free_block_ptr) = free_block {
                // If the current block is a free block
                if ptr_current == free_block_ptr as *mut u8 {
                    unsafe {
                        print(ptr_current.offset_from(mem_ptr) as usize, (*free_block_ptr).size, true);
                        // Move the current pointer forward by the size of the free block (get the next block address)
                        ptr_current = ptr_current.add((*free_block_ptr).size);
                        // Move to the next free block in the list
                        free_block = (*free_block_ptr).next;
                    }
                } else {
                    // If the current block is an allocated block
                    unsafe {
                        let meta_block_ptr = ptr_current as *mut MemMetaBlock;
                        print(ptr_current.offset_from(mem_ptr) as usize + std::mem::size_of::<MemMetaBlock>(), (*meta_block_ptr).size, false);
                        // Move the current pointer forward by the size of the allocated block + metadata
                        ptr_current = ptr_current.add((*meta_block_ptr).size + std::mem::size_of::<MemMetaBlock>());
                    }
                }
            } else {
                // If there are no more free blocks, the rest are occupied
                unsafe {
                    let meta_block_ptr = ptr_current as *mut MemMetaBlock;
                    print(ptr_current.offset_from(mem_ptr) as usize + std::mem::size_of::<MemMetaBlock>(), (*meta_block_ptr).size, false);
                    // Move the current pointer forward by the size of the allocated block + metadata
                    ptr_current = ptr_current.add((*meta_block_ptr).size + std::mem::size_of::<MemMetaBlock>());
                }
            }
        }
    }

    /// Replaces an old free block with a new free block in the free list. (ex: after an alloc smaller than the block)
    pub fn replace(old_block: *mut MemFreeBlock, new_block: *mut MemFreeBlock) {
        unsafe {
            if old_block == new_block {
                return; // If the blocks are the same, no need to replace
            }
            // Check if the old block is the first in the list
            if let Some(first_block) = MemFreeBlock::get_first_block() {
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

    /// Deletes a free block from the free list and re-link
    pub fn delete(block_to_delete: *mut MemFreeBlock) {
        unsafe {
            // Check if the block to delete is the first block in the list
            if let Some(first_block) = MemFreeBlock::get_first_block() {
                if first_block == block_to_delete {
                    // If the block to delete is the first block, update the head of the list
                    MemFreeBlock::set_first_block(
                        (*block_to_delete).next.unwrap_or(ptr::null_mut()),
                    );
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
                    // return the suitable block
                    return Some(current_block);
                }
                current_block = block.get_next().unwrap_or(ptr::null_mut());
            }
        }

        // No suitable block found
        None
    }

    // Best Fit Strategy
    pub fn mem_best_fit(first_block: *mut MemFreeBlock, size: usize) -> Option<*mut MemFreeBlock> {
        let mut current_block = first_block;
        let mut best_fit_block: Option<*mut MemFreeBlock> = None;
        let mut smallest_fit_size = usize::MAX;

        // Iterate over free blocks to find the best fit block
        while !current_block.is_null() {
            unsafe {
                let block = &*current_block;
                let block_size = block.get_size();
                if block_size >= size && block_size < smallest_fit_size {
                    best_fit_block = Some(current_block);
                    smallest_fit_size = block_size;
                }
                current_block = block.get_next().unwrap_or(ptr::null_mut());
            }
        }

        // Return the best fit block found, or None if no suitable block was found
        best_fit_block
    }

    // Worst Fit Strategy
    pub fn mem_worst_fit(first_block: *mut MemFreeBlock, size: usize) -> Option<*mut MemFreeBlock> {
        let mut current_block = first_block;
        let mut worst_fit_block: Option<*mut MemFreeBlock> = None;
        let mut largest_fit_size = 0;

        // Iterate over free blocks to find the worst fit block
        while !current_block.is_null() {
            unsafe {
                let block = &*current_block;
                let block_size = block.get_size();
                if block_size >= size && block_size > largest_fit_size {
                    worst_fit_block = Some(current_block);
                    largest_fit_size = block_size;
                }
                current_block = block.get_next().unwrap_or(ptr::null_mut());
            }
        }

        // Return the worst fit block found, or None if no suitable block was found
        worst_fit_block
    }

    // Insert a new block to the list
    pub fn insert(new_block: *mut MemFreeBlock) {
        unsafe {
            println!("Inserting block at address: {:?}, size: {}", new_block, (*new_block).size);

            if let Some(mut current_block) = MemFreeBlock::get_first_block() {
                // Check if the new block should be the new head
                if new_block < current_block {
                    (*new_block).next = Some(current_block);
                    MemFreeBlock::set_first_block(new_block);
                    return;
                }
                // Traverse the list to find the right position
                while let Some(next_block) = (*current_block).get_next() {
                    if new_block < next_block {
                        // Insert the new block between current_block and next_block
                        (*new_block).next = Some(next_block);
                        (*current_block).next = Some(new_block);
                        return;
                    }
                    current_block = next_block;
                }
                // If we've reached the end of the list, append the new block
                (*current_block).next = Some(new_block);
                (*new_block).next = None;
            } else {
                // If the list is empty, the new block becomes the head
                MemFreeBlock::set_first_block(new_block);
                (*new_block).next = None;
            }
        }
    }

    // Fusion method to merge adjacent free blocks
    pub fn fusion() {
        unsafe {
            if let Some(mut current_block) = MemFreeBlock::get_first_block() {
                // Iterate through the free list and merge adjacent blocks
                while let Some(next_block) = (*current_block).get_next() {
                    let current_block_end =
                        (current_block as *mut u8).add((*current_block).get_size());
                    // Check if the next block is adjacent
                    if current_block_end == next_block as *mut u8 {
                        // Merge the current block with the next block
                        let merged_size = (*current_block).get_size() + (*next_block).get_size();
                        println!("Merging blocks at address: {:?} and {:?}, new size: {}", current_block, next_block, merged_size);
                        (*current_block).set_size(merged_size);
                        // Remove the next block from the list
                        (*current_block).next = (*next_block).get_next();
                    } else {
                        // Move to the next block if they are not adjacent
                        current_block = next_block;
                    }
                }
            }
        }
    }

    pub fn print_free_list() {
        let mem_ptr = mem_space_get_addr();
        let mut current = Self::get_first_block();
        println!("Free List:");
        while let Some(current_block) = current {
            unsafe {
                println!(
                    "Adresse: {:?}, Taille: {}",
                    (current_block as *mut u8).offset_from(mem_ptr) as usize,
                    (*current_block).size
                );
                current = (*current_block).next;
            }
        }
        println!("\n");
    }
}

impl MemMetaBlock {
    // Allocate memory
    pub fn mem_alloc(mut size: usize) -> *mut u8 {
        size = get_modulo(size);

        // Check if there is a fit handler set
        if let Some(handler) = unsafe { FIT_HANDLER } {
            // Get the first free block
            if let Some(first_free_block) = MemFreeBlock::get_first_block() {
                let total_alloc_size = size + std::mem::size_of::<MemMetaBlock>();
                // Call the fit handler
                if let Some(suitable_block) = handler(first_free_block, total_alloc_size)
                {
                    let suitable_block_ptr = suitable_block as *mut MemFreeBlock;
                    // Get the size of the suitable free block
                    let suitable_block_size = unsafe { (*suitable_block_ptr).size };
                    // Calculate the leftover size after allocation
                    let leftover_size = suitable_block_size - total_alloc_size;

                    if leftover_size > std::mem::size_of::<MemMetaBlock>() {
                        // Pointer arithmetic to calculate the address of the new free block after the new allocated block
                        let new_free_block_ptr = unsafe {
                            (suitable_block_ptr as *mut u8).add(total_alloc_size)
                                as *mut MemFreeBlock
                        };
                        // Initialize the new free block
                        unsafe {
                            (*new_free_block_ptr).set_size(leftover_size);
                        }
                        // Update free list
                        MemFreeBlock::replace(suitable_block, new_free_block_ptr);
                    } else {
                        // If no split is needed (size allocated == suitable block size), just remove the block
                        MemFreeBlock::delete(suitable_block);
                        // Adjust the size to include the leftover
                        size = suitable_block_size - std::mem::size_of::<MemMetaBlock>();
                    }

                    // Create and set the size of the allocated MemMetaBlock
                    let meta_block_ptr = (suitable_block_ptr as *mut u8) as *mut MemMetaBlock;
                    unsafe {
                        (*meta_block_ptr).size = size; // Set the size in the metadata block
                    }

                    // Return the pointer to the allocated memory (just after the metadata)
                    return unsafe {
                        (meta_block_ptr as *mut u8).add(std::mem::size_of::<MemMetaBlock>())
                    };
                }
            }
        }

        // Return null if allocation can't be done
        ptr::null_mut()
    }

    // Free memory
    pub fn mem_free(ptr: *mut u8) {
        if ptr.is_null() {
            return;
        }

        // Calculate the address of the metadata block
        let meta_block_ptr = unsafe { (ptr as *mut u8).sub(std::mem::size_of::<MemMetaBlock>()) as *mut MemMetaBlock };

        // Get the size of the block to be freed
        let block_size = unsafe { (*meta_block_ptr).size } + std::mem::size_of::<MemMetaBlock>();

        // Create a new free block
        let free_block_ptr = meta_block_ptr as *mut MemFreeBlock;
        unsafe {
            (*free_block_ptr).size = block_size;
        }

        // Add the block back to the free list
        MemFreeBlock::insert(free_block_ptr);

        // Coalesce adjacent free blocks
        MemFreeBlock::fusion();
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
