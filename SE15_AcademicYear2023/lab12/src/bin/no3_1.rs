// Lab 12
// No 3.1
enum Shape {
    Circle {
        x: i32, 
        y: i32, 
        radius: f32 },
    Rectangle {
        x: i32, 
        y: i32, 
        width: f32, 
        height: f32
    },
}

impl Shape {
    fn rep_string(&self) -> String {
        match self {
            Shape::Circle{x, y, radius} => format!("<Circle: {}, {}, {}>", x, y, radius),
            Shape::Rectangle{x, y, width, height} => format!("<Rectangle: {}, {}, {}, {}>", x, y, width, height),
        }
    }
    fn area(&self) -> f32 {
        match self {
            Shape::Circle{radius, ..} => 3.14 * radius * radius,
            Shape::Rectangle{width, height, ..} => width * height,
        }
    }
}

fn main() {
    let circle1 = Shape::Circle {
        x: 20, 
        y: 20, 
        radius: 30.0
    };
    println!("{}", circle1.area());
    println!("{}", circle1.rep_string());
}

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle { x: 0, y: 0, radius: 1.0 },
    Shape::Circle { x: 50, y: 50, radius: 15.0 },
    Shape::Rectangle { x: 40, y: 40, width: 20.0, height: 20.0 },
    Shape::Rectangle { x: 10, y: 40, width: 15.0, height: 10.0 },
];

const EXPECTED: &[&str] = &[
"<Circle: 0, 0, 1>, area: 3.14",
"<Circle: 50, 50, 15>, area: 706.50",
"<Rectangle: 40, 40, 20, 20>, area: 400.00",
"<Rectangle: 10, 40, 15, 10>, area: 150.00"
];

#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
    |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}