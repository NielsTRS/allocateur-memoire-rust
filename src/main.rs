//------------------------------------------------------------------------------
// Projet : TP CSE (malloc)
// Cours  : Conception des systèmes d'exploitation et programmation concurrente
// Cursus : Université Grenoble Alpes - UFRIM²AG - Master 1 - Informatique
//------------------------------------------------------------------------------

pub mod mem;
pub mod mem_space;

use std::assert;
use std::io::{self, Write};
use std::process::exit;
use std::ptr;

use mem::*;
use mem_space::*;

const MAX_ALLOCATIONS: usize = 128;

fn aide() {
    eprintln!("Aide :");
    eprintln!("Saisir l'une des commandes suivantes\n");
    eprintln!("a taille  :   allouer un bloc de la taille souhaitee");
    eprintln!("r n       :   reallouer le bloc alloue lors de la n-ieme allocation");
    eprintln!("l adresse :   liberer un bloc alloue precedemment a adresse");
    eprintln!("f n       :   liberer le bloc alloue lors de la n-ieme allocation");
    eprintln!("i         :   afficher la liste des emplacements memoire inoccupes");
    eprintln!("o         :   afficher la liste des emplacements memoire occupees");
    eprintln!(
        "M         :   afficher la liste de tous les emplacements memoire (libres et occupes)"
    );
    eprintln!("m         :   afficher le dump de la memoire");
    eprintln!("h         :   afficher cette aide");
    eprintln!("q         :   quitter ce programme\n");
}

fn afficher_zone(adresse: usize, taille: usize, free: bool) {
    println!(
        "Zone {}, Adresse : {}, Taille : {}",
        if free { "libre" } else { "occupee" },
        adresse,
        taille
    );
}

fn afficher_zone_libre(adresse: usize, taille: usize, free: bool) {
    if free {
        afficher_zone(adresse, taille, true);
    }
}

fn afficher_zone_occupee(adresse: usize, taille: usize, free: bool) {
    if !free {
        afficher_zone(adresse, taille, false);
    }
}

fn main() {
    let mut buffer = String::new();
    let mut allocations: [*mut u8; MAX_ALLOCATIONS] = [ptr::null_mut(); MAX_ALLOCATIONS];
    let mut nb_alloc = 0;

    aide();
    MemFreeBlock::mem_init();

    loop {
        print!("? ");
        io::stdout().flush().unwrap();
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut chars = buffer.chars();
        let commande = chars.next().unwrap_or('\n');

        match commande {
            'a' => {
                let taille: isize = match chars.as_str().trim().parse() {
                    Ok(t) => t,
                    Err(_) => {
                        println!("Erreur : taille invalide");
                        continue;
                    }
                };
                if taille < 0 {
                    println!("Erreur : la taille ne peut pas être négative");
                    continue;
                }
                let ptr = MemMetaBlock::mem_alloc(taille as usize);
                allocations[nb_alloc] = ptr;
                nb_alloc += 1;
                if ptr.is_null() {
                    println!("Echec de l'allocation");
                } else {
                    let offset = ptr as isize - mem_space_get_addr() as isize;
                    println!("Memoire allouee en {}", offset);
                }
            }
            'l' => {
                let offset: isize = chars.as_str().trim().parse().unwrap();
                unsafe {
                    MemMetaBlock::mem_free(mem_space_get_addr().offset(offset));
                }
                println!("Memoire liberee");
            }
            'f' => {
                let offset: usize = chars.as_str().trim().parse().unwrap();
                assert!(offset < MAX_ALLOCATIONS);
                MemMetaBlock::mem_free(allocations[offset - 1]);
                allocations[offset - 1] = ptr::null_mut();
                println!("Memoire liberee");
            }
            'r' => {
                // get offset and size
                let input: Vec<&str> = chars.as_str().trim().split_whitespace().collect();
                if input.len() != 2 {
                    println!("Erreur : commande invalide");
                    continue;
                }
                let offset: usize = match input[0].parse() {
                    Ok(o) => o,
                    Err(_) => {
                        println!("Erreur : offset invalide");
                        continue;
                    }
                };
                let taille: isize = match input[1].parse() {
                    Ok(t) => t,
                    Err(_) => {
                        println!("Erreur : taille invalide");
                        continue;
                    }
                };
                if taille < 0 {
                    println!("Erreur : la taille ne peut pas être négative");
                    continue;
                }
                assert!(offset < MAX_ALLOCATIONS);
                let ptr = MemMetaBlock::mem_realloc(allocations[offset - 1], taille as usize);
                allocations[offset - 1] = ptr;
                println!("Memoire reallouee");
            }
            'i' => {
                MemFreeBlock::mem_show(afficher_zone_libre);
            }
            'o' => {
                MemFreeBlock::mem_show(afficher_zone_occupee);
            }
            'M' => {
                MemFreeBlock::mem_show(afficher_zone);
            }
            'm' => {
                print!("[ ");
                let adresse = mem_space_get_addr();
                let size = mem_space_get_size();
                for i in 0..size {
                    print!("{} ", unsafe { *adresse.offset(i as isize) });
                }
                println!("]");
            }
            'h' => {
                aide();
            }
            'q' => {
                exit(0);
            }
            _ => {
                eprintln!("Commande inconnue !");
            }
        }
    }
}
