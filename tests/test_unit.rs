use lib::mem::get_modulo;

#[test]
fn run() {
    let addr = get_modulo(7);
    assert_eq!(addr, 8);
    
    let addr = get_modulo(8);
    assert_eq!(addr, 8);
}