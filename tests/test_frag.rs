//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use lib::mem::*;
use lib::mem_space::{mem_space_get_addr, mem_space_get_size};
use rand::Rng;
use std::env;
use std::ptr;

const MAX_ALLOC: usize = 100000;
const MAX_BLOC: usize = 200;
const FREQ_FREE: usize = 3;

static mut ALLOCS: [*mut u8; MAX_ALLOC] = [ptr::null_mut(); MAX_ALLOC];

#[test]
fn test_frag() {
    MemFreeBlock::mem_init();

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "first" => {
                MemFreeBlock::mem_set_fit_handler(MemFreeBlock::mem_first_fit);
                println!("Stratégie first fit");
            }
            "best" => {
                MemFreeBlock::mem_set_fit_handler(MemFreeBlock::mem_best_fit);
                println!("Stratégie best fit");
            }
            "worst" => {
                MemFreeBlock::mem_set_fit_handler(MemFreeBlock::mem_worst_fit);
                println!("Stratégie worst fit");
            }
            _ => {}
        }
    }

    let mut rng = rand::thread_rng();
    eprintln!(
        "Test réalisant des series d'allocations / désallocations\n\
        afin d'obtenir une forte fragmentation de manière aléatoire\n"
    );

    let mut i = 0;
    let mut size = rng.gen_range(1..=MAX_BLOC);
    let mut free;

    unsafe {
        while i < MAX_ALLOC && {
            ALLOCS[i] = MemMetaBlock::mem_alloc(size);
            !ALLOCS[i].is_null()
        } {
            println!("{} -------------------------------", i);
            println!(
                "Allocation en {}",
                (ALLOCS[i] as usize) - (mem_space_get_addr() as usize)
            );
            assert!(
                ALLOCS[i] < (mem_space_get_addr() as *mut u8).add(mem_space_get_size()) as *mut u8
            );

            if rng.gen_range(0..FREQ_FREE) == 0 {
                free = rng.gen_range(0..=i);
                println!("Libération {}", free);
                assert!(
                    ALLOCS[free]
                        < (mem_space_get_addr() as *mut u8).add(mem_space_get_size()) as *mut u8
                );
                MemMetaBlock::mem_free(ALLOCS[free]);
                ALLOCS[free] = ptr::null_mut();
            }
            size = rng.gen_range(1..=MAX_BLOC);
            i += 1;
        }

        if MemMetaBlock::mem_alloc(size).is_null() {
            println!(
                "Tentative d'allocation de  {} octets.\n\
                Impossible car la mémoire est trop fragmentée.\n\
                {} blocs ont été alloué (certains ont peut-être été libérés)\n",
                size, i
            );
        } else {
            println!("Le tableau d'allocation est trop petit, augmentez MAX_ALLOC ou MAX_BLOC\n");
        }
    }
}
