/* In this exercise, we will randomly genrate a list of points and use it to setup a layer information
in the program. The first thing you need to do is to define the structure Point for storing a point
with cartesian coordinates (x, y) and the structure Layer which has "name", "color" tag
"#RRGGBBAA" (in hex color code), and a list of "points" contained in the layer. and then use it to
write the following functions. */

/* 3.1) Write the function gen_layer(name, color, rng) that create a Layer with name , color ,
and points which is a list of the Point randomly generated using the random number
generator rng for n points within [20, 50] (inclusive) and for each point (x, y) having
-100 ≤ x ≤ 100 and -100 ≤ y ≤ 100 . Add a unit test for the function gen_layer(name,
color, rng) to ensure the correctness of the function. */

/* 3.2) Write the function gen_layer_list(rng, n) that create a list of n layers each with the
name in the pattern "Layer {i}" , the color in "#RRGGBBAA" (which is randomly chosen),
and the points generated by the same condition as in 3.1). Add a unit test for the function
gen_layer_list(rng, n) to ensure the correctness of the function. */

/* 
3.3) Write a program to generate n layers using the function gen_layer_list(rng, n) from
3.2) and save the generated layers as a CSV data. The program must take the value n and
the output filename from the command-line argument. Write an integration test to ensure
the correctness of the program.
Example CSV output representing layers data
"Layer 1", #FF0000FF, -1, 1, 2, -2
"Layer 2", #00FF00FF, -1.2, 1.5, 1.4, 2
*/

use rand::Rng;
use std::fs::File;
use std::io::Write;
use csv::QuoteStyle;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

fn gen_layer(name: String, color: String, rng: &mut impl Rng) -> Layer {
    let num = rng.gen_range(20..=50);
    let mut points = Vec::new();

    for _i in 0..num {
        let x = rng.gen_range(-100. ..= 100.);
        let y = rng.gen_range(-100. ..= 100.);
        points.push(Point{x, y});
    }
    return Layer{name, color, points};
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

    // No 3.1
    // let name: String = "Layer 1".to_string();
    // let color: String = "#RRGGBBAA".to_string();
    // let mut rng = rand::thread_rng();
    // let layer1 = gen_layer(name, color, &mut rng);
    // for p in layer1.points {
    //     println!("{}, {}", p.x, p.y);
    // }

    // No 3.2
    // let mut rng = rand::thread_rng();
    // let layer_list = gen_layer_list(&mut rng, 10);
    // let mut count = 1;
    // for l in layer_list {
    //     println!("Layer {}: {:?}", count, l);
    //     count += 1;
    // }

    // No 3.3
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse().unwrap_or(0); 
    let output_file = &args[2];

    let mut rng = rand::thread_rng();
    let layers = gen_layer_list(&mut rng, n);
    
    let file = File::create(output_file).expect("Unable to load the file");

    let mut wtr = csv::WriterBuilder::new()
                    .delimiter(b',').quote_style(QuoteStyle::Never)
                    .from_writer(file);
    
    for layer in layers {
        let formatted_points: String = layer.points.iter()
                                                    .map(|point| format!("{}, {}", point.x, point.y))
                                                    .collect::<Vec<String>>()
                                                    .join(", ");
        wtr.serialize((layer.name, layer.color, formatted_points));
    }
    
    wtr.flush().unwrap();
}

#[test]
fn test_layer_1() {
    let (name, color) = ("Layer 1".to_string(), "#RRGGBBAA".to_string());
    let mut rng = rand::thread_rng();
    let layer1 = gen_layer(name, color, &mut rng);

    assert!(layer1.points.len() >= 20 && layer1.points.len() <= 50);
    
    for p in layer1.points {
        assert!(p.x >= -100. && p.x <= 100.);
        assert!(p.y >= -100. && p.y <= 100.);
    }
}

#[test]
fn test_layer_list() {
    let mut rng = rand::thread_rng();
    let layer_list = gen_layer_list(&mut rng, 10);

    for l in layer_list {
        assert!(l.points.len() >= 20 && l.points.len() <= 50);
    }
}
