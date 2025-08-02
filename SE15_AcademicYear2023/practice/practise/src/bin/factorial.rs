fn factorial(n: i32) -> i32{
    let mut fac = 1;
    if n < 1 {
        return 1;
    }
    else {
        for _i in 1..=n {
            fac *= _i;
        }
        return fac;
    }
}

fn factorial_r(n: i32) -> i32 {
    if n < 1 { 
        return 1; 
    }
    else {
        return n * factorial_r(n-1);
    }
}

fn main() {
    let num = 3;
    let fac = factorial(num);
    let fac_r = factorial_r(num);
    println!("{}", fac);
    println!("{}", fac_r);
}
