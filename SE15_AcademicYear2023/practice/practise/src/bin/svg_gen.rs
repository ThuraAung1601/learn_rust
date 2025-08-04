extern crate svg;

use svg::node::element::Circle;
use svg::node::element::Rectangle;
use svg::Document;
use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

impl Layer {
    fn new(name: String, color: String, points: Vec<Point>) -> Self {
        Layer { name, color, points }
    }
}

fn gen_layer<R: Rng>(name: &str, color: &str, rng: &mut R) -> Layer {
    let n = rng.gen_range(20..=50); // Generate a random number of points between 20 and 50 (inclusive)
    let mut points = Vec::new();

    for _ in 0..n {
        let x = rng.gen_range(0.0..=500.0); // Generate random x between 0 and 500
        let y = rng.gen_range(0.0..=500.0); // Generate random y between 0 and 500
        points.push(Point { x, y });
    }

    Layer::new(name.to_string(), color.to_string(), points)
}

fn main() {
    // Create a layer using gen_layer function
    let mut rng = rand::thread_rng();
    let layer = gen_layer("Layer 1", "#FF0000", &mut rng);

    // Create an SVG document
    let mut document = Document::new()
        .set("width", "500")
        .set("height", "500")
        .add(Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "#EEEEEE"));

    // Add circles for each point in the layer
    for point in &layer.points {
        let circle = Circle::new()
            .set("cx", point.x)
            .set("cy", point.y)
            .set("r", "5") // Adjust the radius as needed
            .set("fill", &*layer.color);
        document = document.add(circle);
    }

    // Write the SVG document to stdout
    println!("{}", document);
}
