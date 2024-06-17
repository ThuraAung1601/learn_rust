fn check_even_odd<T>(num: T) {    
    if 
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg: &str = if args.len() < 2 {""} else {&args[1]};

    let age: i32 = arg.parse().unwrap_or(0);

    println!("{}", age);

    if age < 18 {
        println!("Underage");
    } else if age > 18 && age < 32 {
        println!("OK");
    } else {
        println!("Overage");
    }

    for n in -2..2 {
        println!("{} is {}", n , match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "neg",
            _ => "pos"
        } );
    }

    let v1: Vec<_> = vec![1, 2, 3];
    println!("{:?}", v1);
}