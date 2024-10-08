//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use lib::mem::*;
use std::assert;

#[test]
fn test_realloc_smaller() {
    MemFreeBlock::mem_init();
    eprintln!(
        "Test reallocating memory to a smaller size\n\
        Define DEBUG at compilation for more verbose output.\n"
    );

    unsafe {
        // Allocate initial block
        let initial_size = 100;
        let realloc_size = 50;
        let ptr = MemMetaBlock::mem_alloc(initial_size);
        assert!(!ptr.is_null(), "Initial allocation failed");

        // Fill the allocated memory with a pattern
        for i in 0..initial_size {
            *ptr.add(i) = i as u8;
        }

        // Reallocate the block to a smaller size
        let new_ptr = MemMetaBlock::mem_realloc(ptr, realloc_size);
        assert!(!new_ptr.is_null(), "Reallocation failed");

        // Verify the contents of the reallocated memory
        for i in 0..realloc_size {
            assert_eq!(
                *new_ptr.add(i),
                i as u8,
                "Memory content mismatch after reallocation"
            );
        }

        // Free the reallocated memory
        MemMetaBlock::mem_free(new_ptr);
    }
}

#[test]
fn test_realloc_larger() {
    MemFreeBlock::mem_init();
    eprintln!(
        "Test reallocating memory to a larger size\n\
        Define DEBUG at compilation for more verbose output.\n"
    );

    unsafe {
        // Allocate initial block
        let initial_size = 100;
        let realloc_size = 200;
        let ptr = MemMetaBlock::mem_alloc(initial_size);
        assert!(!ptr.is_null(), "Initial allocation failed");

        // Fill the allocated memory with a pattern
        for i in 0..initial_size {
            *ptr.add(i) = i as u8;
        }

        // Reallocate the block to a larger size
        let new_ptr = MemMetaBlock::mem_realloc(ptr, realloc_size);
        assert!(!new_ptr.is_null(), "Reallocation failed");

        // Verify the contents of the reallocated memory
        for i in 0..initial_size {
            assert_eq!(
                *new_ptr.add(i),
                i as u8,
                "Memory content mismatch after reallocation"
            );
        }

        // Fill the rest of the reallocated memory with a new pattern
        for i in initial_size..realloc_size {
            *new_ptr.add(i) = (i * 2) as u8;
        }

        // Verify the new contents
        for i in initial_size..realloc_size {
            assert_eq!(
                *new_ptr.add(i),
                (i * 2) as u8,
                "Memory content mismatch in new area after reallocation"
            );
        }

        // Free the reallocated memory
        MemMetaBlock::mem_free(new_ptr);
    }
}
