#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2 };
    println!("Point : {:?}", p);
    println!("Point : x: {}, y: {}", p.x + 1, p.y);
    println!("Point : {:?}", p);
    p.x += 1;
    println!("Point : {:?}", p);
}
