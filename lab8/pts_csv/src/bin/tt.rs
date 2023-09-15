#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    color: String,
}

impl Point {
    fn new(x:i32, y:i32) -> Point {
        Point {
            x,
            y,
            color: String::new(),
        }
    }
}

fn main() {
    let p1 = Point::new(1, 2);
    println!("{:?}", p1);
}