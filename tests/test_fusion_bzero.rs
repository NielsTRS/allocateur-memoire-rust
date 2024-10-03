//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use std::ptr;
use lib::mem::*;
use lib::mem_space::{mem_space_get_size, mem_space_get_addr};

const MAX_ALLOC: usize = 1 << 10;
const NB_TESTS: usize = 5;

fn relative_adr(adr: *mut u8) -> *mut u8 {
    unsafe { adr.offset(-(mem_space_get_addr() as isize)) }
}

fn my_free(mem: &mut *mut u8) {
    if !mem.is_null() {
        MemMetaBlock::mem_free(*mem);
        println!("Freed {:p}\n", relative_adr(*mem));
        *mem = ptr::null_mut();
    }
}

fn checked_alloc(s: usize) -> *mut u8 {
    let result;
    result = MemMetaBlock::mem_alloc(s);
    assert!(!result.is_null());
    unsafe {
        ptr::write_bytes(result, 0, s);
    }
    println!("Alloced {} bytes at {:p}\n", s, relative_adr(result));
    result
}

fn alloc_max(mut estimate: usize) -> *mut u8 {
    let mut result;
    static mut LAST: usize = 0;

    unsafe {
        while {
            result = MemMetaBlock::mem_alloc(estimate);
            result.is_null()
        } {
            estimate -= 1;
        }
        println!("Alloced {} bytes at {:p}\n", estimate, relative_adr(result));
        if LAST != 0 {
            assert!(estimate == LAST);
        } else {
            LAST = estimate;
        }
    }
    result
}

fn alloc5(ptr: &mut [*mut u8; 5]) {
    ptr[0] = checked_alloc(MAX_ALLOC);
    ptr[1] = checked_alloc(MAX_ALLOC);
    ptr[2] = checked_alloc(MAX_ALLOC);
    ptr[3] = checked_alloc(MAX_ALLOC);
    ptr[4] = alloc_max(mem_space_get_size() - 4 * MAX_ALLOC);
}

fn free5(ptr: &mut [*mut u8; 5]) {
    for i in 0..5 {
        my_free(&mut ptr[i]);
    }
}

#[test]
fn test_fusion_bzero() {
    println!("{}", std::mem::size_of::<*mut u8>());
    let mut ptr: [*mut u8; 5] = [ptr::null_mut(); 5];

    MemFreeBlock::mem_init();
    eprintln!(
        "Test réalisant divers cas de fusion (avant, arrière et double\n\
        Définir DEBUG à la compilation pour avoir une sortie un peu plus verbeuse.\n"
    );
    for _ in 0..NB_TESTS {
        println!("Fusion avant\n");
        alloc5(&mut ptr);
        my_free(&mut ptr[2]);
        my_free(&mut ptr[1]);
        ptr[1] = checked_alloc(2 * MAX_ALLOC);
        free5(&mut ptr);

        println!("Fusion arrière\n");
        alloc5(&mut ptr);
        my_free(&mut ptr[1]);
        my_free(&mut ptr[2]);
        ptr[1] = checked_alloc(2 * MAX_ALLOC);
        free5(&mut ptr);

        println!("Fusion avant/arrière\n");
        alloc5(&mut ptr);
        my_free(&mut ptr[1]);
        my_free(&mut ptr[3]);
        my_free(&mut ptr[2]);
        ptr[1] = checked_alloc(3 * MAX_ALLOC);
        free5(&mut ptr);
    }

    // TEST OK
    println!("TEST OK!!");
}