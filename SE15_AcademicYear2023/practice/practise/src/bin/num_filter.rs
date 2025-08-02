// use rand::Rng;

// fn gen_number<R: Rng> (rng: &mut R, x: f64) -> f64 {
//     rng.gen_range(-x ..= x)
// }

// fn filter_number(v: &[f32]) {
//     for i in v {
//         if *i >= -1. && *i <= 1. {
//             println!("{}", *i);
//         }
//     }
// }

// fn main() {
//     // let mut rng = rand::thread_rng();
//     // println!("{}", gen_number(&mut rng, 10.));

//     // let v = [1., 2., 3., 5., 0.5, -1., -0.4];
//     // filter_number(&v);

//     let args: Vec<String> = std::env::args().collect();
//     let n_arg = if args.len() < 2 {""} else {&args[1]};
//     let n = n_arg.parse().unwrap_or(0);

//     // println!("{}", n);
//     let mut rng = rand::thread_rng();
//     let mut counter = 0;
//     for _ in 0..n {
//         let val = rng.gen_range(-10. ..= 10.);
//         // println!("{}", val);
//         if val >= -1. && val <= 1. {
//             // println!(" within range -1 to 1: {}", val);
//             counter += 1;
//         }
//         else {
//             continue;
//         }
//     }
//     let prob = counter as f32 / n as f32;
//     println!("Probability for {0} is {1}", n, prob);
// }

fn filter_points(pt_list: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    for i in pt_list {
        if (-1. ..= 1.).contains(&i.0) && (-1. ..= 1.).contains(&i.1) {
            // println!("{:?}", i);
            result.push(*i);
        }
    }
    return result;
}

use rand::Rng;
fn gen_points<R: Rng> (rng: &mut R, n: i32) -> Vec<(f32, f32)> {
    let mut point_ls = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(-1. ..= 1.);
        let y = rng.gen_range(-1. ..= 1.);
        point_ls.push((x, y));
    }
    return point_ls;
}

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
}