use rand::Rng;

fn gen_rand<R: Rng>(rng: &mut R, x: f32) -> f32 {
    rng.gen_range(-x ..= x)
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(-1. ..= 1.);
    println!("{}", random_num);

    let random_num2 = gen_rand(&mut rng, 40.);
    println!("{}", random_num2);
}
