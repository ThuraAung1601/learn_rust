fn odd_or_even(num: &f32) -> String {
    if *num%2.0 == 0.0 {
        return "odd".to_string();
    } else {
        return "even".to_string();
    }
}

fn pos_or_neg(num: &f32) -> String {
    if *num == 0.0 {
        return "zero".to_string();
    } else if *num > 0.0 {
        return "positive".to_string();
    } else {
        return "negative".to_string();
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let number = args[1].parse().unwrap_or(0.0);

    println!("{}", odd_or_even(&number));
    println!("{}", pos_or_neg(&number));
}