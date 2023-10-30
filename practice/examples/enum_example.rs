// enum impl

#[derive(Debug)]
enum Shape {
    // variants are struct types
    Circle {
        x: f32,
        y: f32,
        radius: f32
    }, 
    Rectangle {
        x: f32,
        y: f32,
        w: f32,
        h: f32
    }
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle{x, y, radius} => 3.142 * radius * radius,
            Shape::Rectangle{x, y, w, h} => w * h,
            _ => panic!("Error"),
        }
    }
    fn clone(&self) -> Self {
        match self {
            Shape::Circle{x, y, radius} => Shape::Circle{x: *x, y: *y, radius: *radius},
            Shape::Rectangle{x, y, w, h} => Shape::Rectangle{x: *x, y: *y, w: *w, h: *h},
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
enum Location {
    // variants are enum of struct types
    Inside(Point),
    Outside(Point)
}

fn main() {
    let c1 = Shape::Circle{
        x: 1.,
        y: 2.,
        radius: 34.21,
    };

    println!("{:?}", c1);
    println!("{:?}", c1.area());

    let pt1 = Point {
        x: 1.,
        y: 2.
    };
    let locate = Location::Inside(pt1);
    println!("{:?}", locate);

    let cloned = c1.clone();
    println!("{:?}", cloned);
}
