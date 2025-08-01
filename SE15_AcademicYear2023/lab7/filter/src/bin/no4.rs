use rand::Rng;

// no 4.1
fn filter_points(pt_list: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    for i in pt_list {
        let distance = ((i.0 * i.0 + i.1 * i.1) as f32).sqrt();
        if distance <= 1.0 {
                result.push(*i);
        }
    }
    return result;
}

// no 4.2
fn gen_points<R: Rng> (rng: &mut R, n: i32) -> Vec<(f32, f32)> {
    let mut point_ls = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(-1. ..= 1.);
        let y = rng.gen_range(-1. ..= 1.);
        point_ls.push((x, y));
    }
    return point_ls;
}

// no 4.3
fn prob_gen_points<R: Rng> (rng: &mut R, n: i32) -> f32 {
    // let mut point_ls = Vec::new();
    let mut counter = 0;
    for _ in 0..n {
        let x = rng.gen_range(-1. ..= 1.);
        let y = rng.gen_range(-1. ..= 1.);
        let distance = ((x * x + y * y) as f32).sqrt();
        if distance <= 1.0 {
                counter += 1;
        }
        // point_ls.push((x, y));
    }
    let prob = counter as f32 / n as f32;
    return prob;
}

fn main() {
    let pt_ls = [(1.2, 2.), (3., 5.4), (0.4, 0.5), (-1., -5.)];
    let result = filter_points(&pt_ls);
    println!("{:?}", result);

    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse().unwrap_or(1);

    let mut rng = rand::thread_rng();
    let pts = gen_points(&mut rng, n);
    println!("{:?}", pts);

   let prob = prob_gen_points(&mut rng, n);
   println!("{}", prob);

   for _n in [10, 100, 1_000, 10_000, 100_000, 1_000_000] {
        let prob = prob_gen_points(&mut rng, _n);
        println!("N = {} and P = {}", _n, prob);
        println!("P * 4 = {} ", prob*4.);
   }
}


#[test]
fn test_filter_points() {
    let result = filter_points(&[(1.2, 2.), (3., 5.4), (0.4, 0.5), (-1., -5.)]);
    let expected = vec![(0.4, 0.5)];
    assert_eq!(filter_points(&[]), vec![]);
    assert_eq!(result, expected);
}

#[test]
fn test_gen_points() {
    let mut rng = rand::thread_rng();
    let n = gen_points(&mut rng, 3);
    for (_i,_j) in n {
        assert!(_i <= 1. && _i >= -1. && _j <= 1. && _j >= -1.);
    }
}