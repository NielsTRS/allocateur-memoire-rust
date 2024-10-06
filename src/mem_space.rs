//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

use crate::mem::get_modulo;

pub const MEMORY_SIZE: usize = 128000; // You can change the memory size here.
static mut MEMORY: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

//-------------------------------------------------------------
// mem_space_get_addr
//-------------------------------------------------------------
pub fn mem_space_get_addr() -> *mut u8 {
    get_modulo(unsafe { MEMORY.as_mut_ptr() } as usize) as *mut u8
}

//-------------------------------------------------------------
// mem_space_get_size
//-------------------------------------------------------------
/// Return the size of the memory space.
pub fn mem_space_get_size() -> usize {
    MEMORY_SIZE
}
