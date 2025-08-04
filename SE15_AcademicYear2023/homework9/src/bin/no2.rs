use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
struct Circle {
    x: f32, 
    y: f32,
    radius: f32,
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    circles: Vec<Circle>,
}

// No 2.2
fn cal_average_area(layers: Vec<Layer>) -> Vec<(String, f32)> {
    let mut result = Vec::new();
    const PI: f32 = 3.142;
    for l in layers {
        let mut sum = 0.;
        for c in &l.circles {
            // println!("{:?}", c.radius);
            let area = PI * c.radius * c.radius;
            // println!("{}", area);
            sum += area;
        }
        let avg_area = sum/l.circles.len() as f32;
        result.push((l.name, avg_area))
    }
    return result;
}

fn gen_layer(name: String, color: String, rng: &mut impl Rng) -> Layer {
    let mut circles = Vec::new();
    let num = rng.gen_range(20..=50);
    for _ in 0..num {
        let x = rng.gen_range(-100. ..= 100.);
        let y = rng.gen_range(-100. ..= 100.);
        let radius = rng.gen_range(-10. ..= 20.);
        circles.push(Circle {
                        x,
                        y,
                        radius,
                })
    }
    Layer {
        name,
        color,
        circles,
    }
}

fn gen_obj_layer_list(rng: &mut impl Rng, n: usize) -> Vec<Layer> {
    let mut layer_list = Vec::new();
    for i in 0..n {
        let name = format!("Layer {}", i+1);
        let r = rng.gen::<u8>();
        let g = rng.gen::<u8>();
        let b = rng.gen::<u8>();
        let a = rng.gen::<u8>();
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
        layer_list.push(gen_layer(name, color, rng));
    }
    layer_list
}

// No 2.1
fn save_layer(writer: impl Write, pt_list: &[Layer]) {
    let mut wtr = csv::WriterBuilder::new()
                        .has_headers(true)
                        .delimiter(b';')
                        .quote_style(csv::QuoteStyle::Never)
                        .from_writer(writer);
    wtr.write_record(["Name", "Color", "Circle (x,y,radius)"]).unwrap(); 
    // just header of csv
    // without unwrap() it will give Result data 
    // which is Option Enum type and Result could be both Ok() and Err()

    for layer in pt_list {
        let mut circle_infos = String::new();
        for circle in &layer.circles {
                circle_infos.push_str(format!("({}, {}, {})", circle.x, circle.y, circle.radius).as_str());
                circle_infos.push_str(",");
        }
        wtr.serialize((
            &layer.name,
            &layer.color,
            &circle_infos,
        )).unwrap();
        wtr.flush().unwrap();
    }
}

fn load_layer(reader: impl Read) -> Vec<Layer> {
    let mut layer_ls = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(reader);

    for result in rdr.records() {
        let record = result.unwrap();
        if record.len() < 3 {
            continue; 
        }

        // Extract data from the CSV record
        let name = record[0].to_string();
        let color = record[1].to_string();

        // Parse circles from the third column
        let circles_data = record[2].trim_matches(|c| c == '(' || c == ')');
        let mut circles = Vec::new();

        for coords_str in circles_data.split("),(") {
            let coords: Vec<&str> = coords_str.split(',').collect();
            if coords.len() == 3 {
                let x = coords[0].trim().parse().unwrap_or(0.0);
                let y = coords[1].trim().parse().unwrap_or(0.0);
                let radius = coords[2].trim().parse().unwrap_or(0.0);
                circles.push(Circle { x, y, radius });
            }
        }

        // Create a Layer instance and push it to the vector
        let layer = Layer { name, color, circles };
        layer_ls.push(layer);
    }
    layer_ls
}

fn save_circle_average_area(writer: impl Write, area_ls: &[(String, f32)]) {
    let mut wtr = csv::WriterBuilder::new()
                        .has_headers(true)
                        .delimiter(b',')
                        .from_writer(writer);

    wtr.write_record(["Name", "Area Average"]).unwrap(); 
    
    for layer in area_ls {
        wtr.serialize((
            &layer.0,
            layer.1,
        )).unwrap();
        wtr.flush().unwrap();
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let n = 3;

    // No 2.1
    let layer_ls = gen_obj_layer_list(&mut rng, n);
    let output_layer_file = File::create("layers.csv").expect("File cannot be created.");
    save_layer(output_layer_file, &layer_ls);

    // No 2.2 
    let input_file = File::open("layers.csv").expect("File cannot be opened.");
    let layer_ls = load_layer(input_file);
    // println!("{:?}", layer_ls);
    let result = cal_average_area(layer_ls);
    // println!("{:?}", result);
    let output_area_file = File::create("circle_area.csv").expect("File cannot be created.");
    save_circle_average_area(output_area_file, &result);
}