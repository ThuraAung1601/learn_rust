// Generic

fn printer<T: std::fmt::Display + std::fmt::Debug>(num: T) {
    println!("{:?}", num);
}

fn swap<T1, T2>(n1: T1, n2: T2) -> T1 {
    return n1
}

fn main() {
    printer(3.14);
    printer(3);
    printer("Hello");

    swap(3.14, 3);
    printer(swap(3.14, 3));
}