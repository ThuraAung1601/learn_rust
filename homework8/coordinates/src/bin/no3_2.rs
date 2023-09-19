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

    let polar1 = PolarPoint::new(2.82, 0.78);
    let polar2 = PolarPoint::new(5.0, 0.92);
    let polar3 = PolarPoint::new(1.41, -0.78);
    let polar4 = PolarPoint::new(5.0, 1.57);
    
    let polar_pt_list = vec![polar1, polar2, polar3, polar4];
    let cartesian_list = to_cartesian(&polar_pt_list);

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
