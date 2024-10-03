//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use std::ptr;
use std::assert;
use lib::mem::*;

const NB_TESTS: usize = 5;
const NB_MAX_STORES: usize = 100;

fn my_free(mem: &mut *mut u8) {
    if !mem.is_null() {
        MemMetaBlock::mem_free(*mem);
        println!("Freed {:?}\n", *mem);
        *mem = ptr::null_mut();
    }
}

fn checked_alloc(s: usize) -> *mut u8 {
    let result;
    result = MemMetaBlock::mem_alloc(s);
    assert!(!result.is_null());
    println!("Alloced {} bytes at {:?}\n", s, result);
    result
}

static mut FIRST: bool = true;
static mut NB_ALLOCS: usize = 0;
static mut ALLOCS: [*mut u8; NB_MAX_STORES] = [ptr::null_mut(); NB_MAX_STORES];

fn reset() {
    unsafe {
        FIRST = false;
        NB_ALLOCS = 0;
    }
}

fn store_or_check(adr: *mut u8) {
    unsafe {
        if NB_ALLOCS < NB_MAX_STORES {
            if FIRST {
                ALLOCS[NB_ALLOCS] = adr;
                NB_ALLOCS += 1;
            } else {
                assert!(ALLOCS[NB_ALLOCS] == adr);
                NB_ALLOCS += 1;
            }
        }
    }
}

fn alloc_fun(n: isize) {
    if n < 0 {
        return;
    }
    let mut a = checked_alloc(5);
    store_or_check(a);
    let mut b = checked_alloc(10);
    store_or_check(b);
    alloc_fun(n - 1);
    my_free(&mut a);
    let mut c = checked_alloc(5);
    store_or_check(c);
    alloc_fun(n - 2);
    my_free(&mut c);
    my_free(&mut b);
}

#[test]
fn test_cheese() {
    MemFreeBlock::mem_init();
    eprintln!(
        "Test réalisant récursivement une allocation en gruyère selon le modèle d'appel de fibonacci.\n\
        Définir DEBUG à la compilation pour avoir une sortie un peu plus verbeuse.\n"
    );
    for i in 0..NB_TESTS {
        println!("Issuing test number {}\n", i);
        alloc_fun(6);
        reset();
    }
}