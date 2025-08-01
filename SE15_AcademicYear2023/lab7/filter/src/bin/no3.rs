use rand::Rng;

// no 3.1
fn filter_number(v: &[f32]) -> Vec<f32> {
    let mut result = Vec::new();
    for i in v {
        if *i >= -1. && *i <= 1. {
            result.push(*i);
        }
    }
    return result;
}

// no 3.2
fn gen_number<R: Rng> (rng: &mut R, x: f64) -> f64 {
    rng.gen_range(-x ..= x)
}

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", gen_number(&mut rng, 10.));

    // no 3.1
    let v = [1., 2., 3., 5., 0.5, -1., -0.4];
    let f = filter_number(&v);
    println!("{:?}", f);

    // no 3.2
    let args: Vec<String> = std::env::args().collect();
    let n_arg = if args.len() < 2 {""} else {&args[1]};
    let n = n_arg.parse().unwrap_or(0);
    // println!("{}", n);

    // no 3.3
    let mut rng = rand::thread_rng();
    let mut counter = 0;
    for _ in 0..n {
        let val = rng.gen_range(-10. ..= 10.);
        // println!("{}", val);
        if val >= -1. && val <= 1. {
            // println!(" within range -1 to 1: {}", val);
            counter += 1;
        }
        else {
            continue;
        }
    }

    let prob = counter as f32 / n as f32;
    println!("Probability for {0} is {1}", n, prob);

    for _n in [10, 100, 1_000, 10_000, 100_000, 1_000_000] {
        for _ in 0.._n {
            let val = rng.gen_range(-10. ..= 10.);
            // println!("{}", val);
            if val >= -1. && val <= 1. {
                // println!(" within range -1 to 1: {}", val);
                counter += 1;
            }
            else {
                continue;
            }
        }
        let prob = counter as f32 / _n as f32;
        println!("Probability for {0} is {1}", _n, prob);
    }
}

#[test]
fn test_filter_number() {
    let result = filter_number(&[1., 2., 3., 5., 0.5, -1., -0.4]);
    let expected = vec![1.0, 0.5, -1.0, -0.4];
    assert_eq!(filter_number(&[]), vec![]);
    assert_eq!(result, expected);
}

#[test]
fn test_gen_number() {
    let mut rng = rand::thread_rng();
    let n = gen_number(&mut rng, 10.);
    assert!(n <= 10. && n >= -10.);
}