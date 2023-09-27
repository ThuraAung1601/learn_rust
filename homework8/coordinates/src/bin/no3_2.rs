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
    <th style=\"text-align:center\">X</th>
    <th style=\"text-align:center\">Y</th>
    </tr>
    ");

    let polar_input_file = File::open("./polar_input.csv").expect("Unable to open the file");
    let polar_list = load_polar_file(polar_input_file);
    let cartesian_list = to_cartesian(&polar_list);
    
    for _pt in cartesian_list {
        html.push_str(&format!("
            <tr>
            <td style=\"text-align:right\">{:.2}</td>
            <td style=\"text-align:right\">{:.2}</td>
            </tr>
            ",_pt.x, _pt.y))
    }

    // table end
    html.push_str("
    </table>
    ");

    println!("{}", html);    
}
