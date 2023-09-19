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

fn main() {
    // No 1.1
    let p1 = Point::new(2., 2.);
    let p2 = Point::new(3., 4.);
    let p3 = Point::new(-1., 1.);
    let p4 = Point::new(0., 5.);
    
    let pt_list = vec![p1, p2, p3, p4];
    let polar_list = to_polar(&pt_list);

    println!("No 1.1\nCartesian Points");
    for _pt in pt_list {
        println!("{}, {}", _pt.x, _pt.y);
    }

    println!("Polar Points");
    for _pt in polar_list {
        println!("{}, {}", _pt.r, _pt.t);
    }

    println!();
    
    // No 1.2
    let polar1 = PolarPoint::new(2.82, 0.78);
    let polar2 = PolarPoint::new(5.0, 0.92);
    let polar3 = PolarPoint::new(1.41, -0.78);
    let polar4 = PolarPoint::new(5.0, 1.57);
    
    let polar_pt_list = vec![polar1, polar2, polar3, polar4];
    let cartesian_list = to_cartesian(&polar_pt_list);

    println!("No 1.2\nPolar Points");
    for _pt in polar_pt_list {
        println!("{}, {}", _pt.r, _pt.t);
    }

    println!("Cartesian Points");
    for _pt in cartesian_list {
        println!("{}, {}", _pt.x, _pt.y);
    }    
}
