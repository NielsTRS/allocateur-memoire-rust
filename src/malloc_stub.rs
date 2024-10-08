//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use std::ffi::c_void;
use std::ptr;
use std::ptr::copy_nonoverlapping;
use std::sync::Once;

use crate::mem::*;

// For thread-local reentrancy prevention
thread_local! {
    static GBL_IN_LIB: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

// Macro to prevent reentrant printf calls (dprintf)
macro_rules! dprintf {
    ($($arg:tt)*) => {
        GBL_IN_LIB.with(|gbl_in_lib| {
            if !gbl_in_lib.get() {
                gbl_in_lib.set(true);
                println!($($arg)*);
                gbl_in_lib.set(false);
            }
        });
    };
}

// Helper to get the minimum of two values
fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

// Ensure the allocator is initialized only once
static INIT: Once = Once::new();

// Initialization function for the allocator
fn init() {
    INIT.call_once(|| {
        MemFreeBlock::mem_init();
    });
}

// Overriding malloc
#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    // Lazy initialization
    init();

    // Debug print
    dprintf!("Allocation de {} octets...", size);

    // Forward to our custom allocator
    let result = MemMetaBlock::mem_alloc(size);

    // Debug print on failure or success
    if result.is_null() {
        dprintf!(" Alloc FAILED !!");
    } else {
        dprintf!(" {:p}", result);
    }

    result as *mut c_void
}

// Overriding calloc
#[no_mangle]
pub extern "C" fn calloc(count: usize, size: usize) -> *mut c_void {
    let total_size = count * size;

    // Lazy initialization
    init();

    // Debug print
    dprintf!("Allocation de {} octets", total_size);

    // Forward to our custom allocator
    let ptr = MemMetaBlock::mem_alloc(total_size);

    // Debug print on failure
    if ptr.is_null() {
        dprintf!(" Alloc FAILED !!");
    }

    // Zero-initialize the memory if allocation succeeded
    if !ptr.is_null() {
        unsafe { ptr::write_bytes(ptr, 0, total_size) };
    }

    ptr as *mut c_void
}

// Overriding realloc
#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    // Lazy initialization
    init();

    // Debug print
    dprintf!("Allocation de {} octets...", size);

    // Forward to our custom allocator
    let result = MemMetaBlock::mem_realloc(ptr as *mut u8, size);

    // Debug print on failure or success
    if result.is_null() {
        dprintf!(" Realloc FAILED !!");
    } else {
        dprintf!(" {:p}", result);
    }

    result as *mut c_void
}

// Overriding free
#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    // Lazy initialization
    init();

    // Handle the case where the pointer is NULL
    if !ptr.is_null() {
        dprintf!("Liberation de la zone en {:p}", ptr);
        MemMetaBlock::mem_free(ptr as *mut u8);
    } else {
        dprintf!("Liberation de la zone NULL");
    }
}
