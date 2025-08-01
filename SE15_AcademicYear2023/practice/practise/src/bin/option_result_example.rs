// Option
// Result

fn check_even(n: i32) -> Option<i32> {
    if n%2 == 0 {
        Some(n)
    }
    else {
        None
    }
}

fn extract_even(opt: Vec<Option<i32>>) {
    for i in opt {
        match i {
            Some(i) => println!("{} is even.", i),
            None => println!("Passed.")
        }
    }
}

fn check_odd(n: i32) -> Result<i32, String> {
    if n%2 != 0 {
        Ok(n)
    }
    else {
        Err("This is not odd.".to_string())
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut opt: Vec<Option<i32>> = Vec::new();
    for i in v1 {
        // println!("{:?}", check_even(i));
        opt.push(check_even(i));
    }
    // println!("{:?}", opt);
    extract_even(opt);

    for i in v1 {
        println!("{:?}", check_odd(i));
    }
}