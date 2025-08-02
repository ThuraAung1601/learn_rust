use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::io::Read;

struct Point {
    x: f64,
    y: f64,
}

struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

fn gen_layer(name: String, color: String, rng: &mut impl Rng) -> Layer {
    let n = rng.gen_range(20..=50); // Generate a random number of points between 20 and 50 (inclusive)
    let mut points = Vec::new();

    for _ in 0..n {
        let x = rng.gen_range(0.0..=500.0); // Generate random x between 0 and 500
        let y = rng.gen_range(0.0..=500.0); // Generate random y between 0 and 500
        points.push(Point { x, y });
    }
    
    Layer { name, color, points }
}

fn gen_layer_list(rng: &mut impl Rng, n: i32) -> Vec<Layer> {
    let mut layer_list = Vec::new();
    for i in 0..n {
        let name = format!("Layer {}", i+1);
        // Generate random values for R, G, B, and A components
        let r = rng.gen::<u8>();
        let g = rng.gen::<u8>();
        let b = rng.gen::<u8>();
        let a = rng.gen::<u8>();
        // Format the components as hexadecimal strings with two digits
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a); 
        layer_list.push(gen_layer(name, color, rng));
    }
    layer_list
}

fn main() {
    // No. 4.1
    // Create a layer using gen_layer function
    // let mut rng = rand::thread_rng();
    // let layer = gen_layer("Layer 1".to_string(), "#FF0000".to_string(), &mut rng);

    // let mut document = "<svg height=\"500\" width=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n".to_string();
    // document.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />\n");

    // for point in layer.points {
    //     let circle = format!("<circle cx=\"{}\" cy=\"{}\" fill=\"{}\" r=\"5\"/>\n", point.x, point.y, layer.color);
    //     document.push_str(&circle);
    // }

    // document.push_str("</svg>");
    // // Write the SVG document to stdout
    // println!("{}", document);

    // No 4.2
    let mut rng = rand::thread_rng();
    let layer_list = gen_layer_list(&mut rng, 10);
    let mut count = 1;
    for layer in layer_list {
        // println!("Layer {}: {:?}", count, l);
        let mut document = "<svg height=\"500\" width=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n".to_string();
        document.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />\n");
        for point in layer.points {
                let circle = format!("<circle cx=\"{}\" cy=\"{}\" fill=\"{}\" r=\"5\"/>\n", point.x, point.y, layer.color);
                document.push_str(&circle);
        }
        document.push_str("</svg>");
        // println!("{}", document);
        let file_name = format!("layer_{}.svg",count);
        let mut file = File::create(file_name).expect("Unable to create a file.");
        file.write_all(document.as_bytes());
        count += 1;
    }
}