//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------



use std::alloc::{GlobalAlloc, Layout};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::copy_nonoverlapping;

use mem::*;
use mem_space::*;

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
    if a < b { a } else { b }
}

// Ensure the allocator is initialized only once
static INIT: Once = Once::new();

// Initialization function for the allocator
fn init() {
    INIT.call_once(|| {
        mem_init();
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
    let result = mem_alloc(size);

    // Debug print on failure or success
    if result.is_null() {
        dprintf!(" Alloc FAILED !!");
    } else {
        dprintf!(" {:p}", result);
    }

    result
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
    let ptr = mem_alloc(total_size);

    // Debug print on failure
    if ptr.is_null() {
        dprintf!(" Alloc FAILED !!");
    }

    // Zero-initialize the memory if allocation succeeded
    if !ptr.is_null() {
        unsafe { ptr::write_bytes(ptr, 0, total_size) };
    }

    ptr
}

// Overriding realloc
#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    // Lazy initialization
    init();

    // Debug print
    dprintf!("Reallocation de la zone en {:p}", ptr);

    if ptr.is_null() {
        dprintf!(" Realloc of NULL pointer");
        return mem_alloc(size);
    }

    // Get the current size of the allocated block
    let current_size = mem_get_size(ptr);

    // If the current size is sufficient, return the same pointer
    if current_size >= size {
        dprintf!(" Useless realloc");
        return ptr;
    }

    // Allocate a new block
    let result = mem_alloc(size);
    if result.is_null() {
        dprintf!(" Realloc FAILED");
        return ptr::null_mut();
    }

    // Calculate the size to copy and perform the copy
    let copy_size = min(size, current_size);
    unsafe { copy_nonoverlapping(ptr as *const u8, result as *mut u8, copy_size) };

    // Free the old block
    mem_free(ptr);

    dprintf!(" Realloc ok");

    result
}

// Overriding free
#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    // Lazy initialization
    init();

    // Handle the case where the pointer is NULL
    if !ptr.is_null() {
        dprintf!("Liberation de la zone en {:p}", ptr);
        mem_free(ptr);
    } else {
        dprintf!("Liberation de la zone NULL");
    }
}

// Dummy implementations for memory allocation functions to be replaced with actual logic
fn mem_init() {
    // Custom memory allocator initialization logic here
}

fn mem_alloc(size: usize) -> *mut c_void {
    // Replace this with your custom memory allocation logic
    ptr::null_mut()
}

fn mem_get_size(ptr: *mut c_void) -> usize {
    // Replace this with the logic to get the size of the allocated block
    0
}

fn mem_free(ptr: *mut c_void) {
    // Replace this with your custom memory free logic
}
