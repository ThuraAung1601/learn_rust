use rand::Rng;

// unit circle 
fn filter_pts(pts: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut filtered_pts = Vec::new();
    for i in pts {
        let distance = ((i.0*i.0) + (i.1*i.1)).sqrt();
        if distance <= 1.0 {
            filtered_pts.push(*i);
        }
    }
    filtered_pts
}

fn gen_pts<R: Rng>(rng: &mut R, n: usize) -> Vec<(f32, f32)> {
    let mut generated = Vec::new();
    for _i in 0..n {
        let x = rng.gen_range(-1. ..= 1.);
        let y = rng.gen_range(-1. ..= 1.);
        generated.push((x, y));
    }
    generated
}

fn main() {
    let pt_list = vec![(0.1, 0.2), (1., 2.)];
    println!("{:?}", filter_pts(&pt_list));

    let mut rng = rand::thread_rng();

    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<usize>().unwrap_or(0);

    let generated = gen_pts(&mut rng, n);
    println!("{:?}", generated);

    let unit_circle_pts = filter_pts(&generated);
    println!("{:?}", unit_circle_pts);
}