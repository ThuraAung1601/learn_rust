use std::fs::File;
use std::io::Read;

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

fn main() {

    let mut html: String = String::new();

    // No 3.1

    // table start
    html.push_str("
    <table>
    ");

    // table head
    html.push_str("
    <tr>
    <th style=\"text-align:center\">Radian</th>
    <th style=\"text-align:center\">Theta</th>
    </tr>
    ");

    let cartesian_input_file = File::open("./cartesian_input.csv").expect("Unable to open the file");
    let cartesian_list = load_cartesian_file(cartesian_input_file);
    let polar_list = to_polar(&cartesian_list);
    
    for _pt in polar_list {
        html.push_str(&format!("
            <tr>
            <td style=\"text-align:right\">{:.2}</td>
            <td style=\"text-align:right\">{:.2}</td>
            </tr>
            ",_pt.r, _pt.t))
    }

    // table end
    html.push_str("
    </table>
    ");

    println!("{}", html);
}
