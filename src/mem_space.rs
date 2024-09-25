//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

pub const MEMORY_SIZE: usize = 4096; // You can change the memory size here.

static mut MEMORY: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

//-------------------------------------------------------------
// mem_space_get_addr
//-------------------------------------------------------------
/// Return the address of the memory space.
pub fn mem_space_get_addr() -> *mut u8 {
    unsafe { MEMORY.as_mut_ptr() } // Access to static mutable memory requires 'unsafe' in Rust
}

//-------------------------------------------------------------
// mem_space_get_size
//-------------------------------------------------------------
/// Return the size of the memory space.
pub fn mem_space_get_size() -> usize {
    MEMORY_SIZE
}
