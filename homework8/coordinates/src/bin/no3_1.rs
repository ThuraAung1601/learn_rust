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

    let p1 = Point::new(2., 2.);
    let p2 = Point::new(3., 4.);
    let p3 = Point::new(-1., 1.);
    let p4 = Point::new(0., 5.);
    
    let pt_list = vec![p1, p2, p3, p4];
    let polar_list = to_polar(&pt_list);

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
