//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use lib::mem::*;
use std::assert;
use std::ptr;

const MAX_ALLOC: usize = 1 << 10;
const NB_TESTS: usize = 10;

static mut ALLOCS: [*mut u8; MAX_ALLOC] = [ptr::null_mut(); MAX_ALLOC];

fn make_test() -> usize {
    let nb_alloc;
    let mut i: isize = 0;
    // On remplit la mémoire de blocs de taille croissante
    println!("Issuing a sequence of size increasing mallocs, starting from 0");
    unsafe {
        while (i as usize) < MAX_ALLOC && {
            ALLOCS[i as usize] = MemMetaBlock::mem_alloc(i as usize);
            !ALLOCS[i as usize].is_null()
        } {
            i += 1;
        }
        i -= 1;

        println!("Alloced up to {} bytes at {:?}", i, ALLOCS[i as usize]);

        nb_alloc = i as usize;

        // On vide
        println!("Freeing all allocated memory");
        while i >= 0 {
            MemMetaBlock::mem_free(ALLOCS[i as usize]);
            println!("Freed {:?}", ALLOCS[i as usize]);
            i -= 1;
        }
    }
    nb_alloc
}

#[test]
fn test_base() {
    MemFreeBlock::mem_init();
    eprintln!(
        "Test réalisant des series d'allocations / désallocations en ordre LIFO\n\
        Définir DEBUG à la compilation pour avoir une sortie un peu plus verbeuse.\n"
    );
    let nb_alloc = make_test();
    for _ in 0..NB_TESTS {
        assert!(make_test() == nb_alloc);
    }
}
