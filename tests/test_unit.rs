use lib::mem::{get_modulo, MODULO};
use rand::Rng;

#[test]
fn modulo_7() {
    let addr = get_modulo(7);
    assert_eq!(addr, 8);
}

#[test]
fn modulo_8() {
    let addr = get_modulo(8);
    assert_eq!(addr, 8);
}

#[test]
fn modulo_9() {
    let addr = get_modulo(9);
    assert_eq!(addr, 16);
}

#[test]
fn modulo_rand() {
    let addr = get_modulo(rand::thread_rng().gen_range(1..100));
    assert_eq!(addr % MODULO , 0);
}