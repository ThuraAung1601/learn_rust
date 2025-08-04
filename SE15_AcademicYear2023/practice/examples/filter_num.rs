use rand::Rng;

fn filter_num(num_list: &[f32]) -> Vec<f32> {
    let mut filtered = Vec::new();
    for i in num_list {
        if *i >= -1. && *i <= 1. {
            filtered.push(*i);
        }
    }
    filtered
}

fn filter_num2(num_list: &[f32]) -> Vec<f32> {
    num_list.iter().filter(|&&s| s >= -1. && s <= 1.).cloned().collect()
}

fn gen_numbers<R: Rng>(rng: &mut R, n: usize) -> Vec<f32> {
    let mut num_ls = Vec::new();
    for i in 0..n {
        let rand = rng.gen_range(-10. ..= 10.);
        num_ls.push(rand);
    }
    num_ls
}

fn main() {
    let v1 = vec![0.1, 2.2, 1.1, 0.2, 3.5];
    let filtered = filter_num(&v1);
    println!("{:?}", filtered);

    // let args: Vec<String> = std::env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("Should add number list");
    //     std::process::exit(1);
    // }
    // let v2: Vec<f32> = args.iter().skip(1).map(|s| s.parse::<f32>().unwrap_or(0.0)).collect();
    // // println!("{:?}", v2);
    // let filtered2 = filter_num(&v2);
    // println!("{:?}", filtered2);
    // let filtered3 = filter_num2(&v2);
    // println!("{:?}", filtered3);

    let mut rng = rand::thread_rng();
    let n = 1000;
    let generated = gen_numbers(&mut rng, n);
    // println!("{:?}", generated);

    let count = generated.iter().filter(|&&s| s >= -1. && s <= 1.).cloned().count();
    println!("{}", count as f32 / n as f32);
}

#[test]
fn test_filter_number() {
    let v1 = vec![0.1, 2.2, 1.1, 0.2, 3.5];
    let filtered = filter_num(&v1);
    assert_eq!(vec![0.1, 0.2], filtered);
}