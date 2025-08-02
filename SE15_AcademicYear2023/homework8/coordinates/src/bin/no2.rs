use std::io::Read;
use std::io::Write;
use std::fs::File;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y:f32) -> Point {
        Point {
            x,
            y,
        }
    }
}

struct PolarPoint {
    r: f32,
    t: f32,
}

impl PolarPoint {
    fn new(r: f32, t: f32) -> PolarPoint {
        PolarPoint {
            r,
            t,
        }
    }
}

// No 1.1
// (r,θ): r = √ (x^2+y^2) && θ = tan^-1 (y/x)
fn to_polar(pt_list: &[Point]) -> Vec<PolarPoint> {
    let mut polar_pt_list = Vec::new();
    for point in pt_list {
        let _r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let _t = (point.y / point.x).atan();
        let _polar = PolarPoint::new(_r, _t);
        polar_pt_list.push(_polar);
    }
    polar_pt_list
}

// No 1.2
// x = r * cos(t) && y = r * sin(t)
fn to_cartesian(pt_list: &[PolarPoint]) -> Vec<Point> {
    let mut cartesian_pt_list = Vec::new();
    for point in pt_list {
        let _x = point.r * point.t.cos();
        let _y = point.r * point.t.sin();
        let _cartesian = Point::new(_x, _y);
        cartesian_pt_list.push(_cartesian);
    }
    cartesian_pt_list
}

// No 2.1
fn load_cartesian_file(reader: impl Read) -> Vec<Point> {
    let mut pt_list = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
                        .has_headers(false)
                        .from_reader(reader);

    for r in rdr.records() {
        let record = r.unwrap();
        if record.len() < 2 {
            continue;
        }
        let _x: f32 = record[0].parse().unwrap_or(0.0);
        let _y: f32 = record[1].parse().unwrap_or(0.0);
        // println!("{},{}", _x, _y);
        let _point = Point::new(_x, _y);
        pt_list.push(_point);
    }
    pt_list
}

fn save_polar_file(writer: impl Write, pt_list: &[PolarPoint]) {
    let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false)
                    .delimiter(b',')
                    .from_writer(writer);
    
    for pt in pt_list {
        wtr.serialize((pt.r, pt.t)).unwrap();
    }

    wtr.flush().unwrap();
}

// No 2.2
fn load_polar_file(reader: impl Read) -> Vec<PolarPoint> {
    let mut pt_list = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
                        .has_headers(false)
                        .from_reader(reader);

    for r in rdr.records() {
        let record = r.unwrap();
        if record.len() < 2 {
            continue;
        }
        let _r: f32 = record[0].parse().unwrap_or(0.0);
        let _t: f32 = record[1].parse().unwrap_or(0.0);
        // println!("{},{}", _r, _t);
        let _point = PolarPoint::new(_r, _t);
        pt_list.push(_point);
    }
    pt_list
}

fn save_cartesian_file(writer: impl Write, pt_list: &[Point]) {
    let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false)
                    .delimiter(b',')
                    .from_writer(writer);
    
    for pt in pt_list {
        wtr.serialize((pt.x, pt.y)).unwrap();
    }

    wtr.flush().unwrap();
}

fn main() {

    // No 2.1
    let cartesian_input_file = File::open("./cartesian_input.csv").expect("Unable to open the file");
    let cartesian_list = load_cartesian_file(cartesian_input_file);
    let polar_list = to_polar(&cartesian_list);
    let polar_output_file = File::create("./polar_output.csv").expect("Unable to create the file");
    save_polar_file(polar_output_file, &polar_list);

    // No 2.2
    let polar_input_file = File::open("./polar_input.csv").expect("Unable to open the file");
    let polar_list = load_polar_file(polar_input_file);
    let cartesian_list = to_cartesian(&polar_list);
    let cartesian_output_file = File::create("./cartesian_output.csv").expect("Unable to create the file");
    save_cartesian_file(cartesian_output_file, &cartesian_list);
}
