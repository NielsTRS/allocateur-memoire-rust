//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use lib::mem::*;
use lib::mem_space::mem_space_get_size;

const NB_TESTS: usize = 10;

// Function to allocate maximum memory
fn alloc_max(mut estimate: usize) -> *mut u8 {
    let mut result: *mut u8;
    static mut LAST: usize = 0; // Keep track of last allocation size for idempotence test

    unsafe {
        // Loop to find the maximum amount of memory we can allocate
        while {
            result = MemMetaBlock::mem_alloc(estimate);
            result.is_null()
        } {
            estimate -= 1; // Decrease the estimate if the allocation fails
        }

        // Debug output if compiled in debug mode
        println!("Allocated {} bytes at {:?}", estimate, result);

        // Idempotence test
        if LAST != 0 {
            assert_eq!(
                estimate, LAST,
                "Idempotence test failed: sizes do not match."
            );
        } else {
            LAST = estimate; // Set the initial allocation size
        }

        result
    }
}

#[test]
fn test_init() {
    eprintln!(
        "Test réalisant de multiples fois une initialisation \
        suivie d'une alloc max.\nDéfinir DEBUG à la compilation pour avoir \
        une sortie un peu plus verbeuse.\n"
    );

    // Run the test NB_TESTS times
    for _ in 0..NB_TESTS {
        MemFreeBlock::mem_init(); // Initialize the memory system
        alloc_max(mem_space_get_size()); // Allocate maximum memory
    }
}
