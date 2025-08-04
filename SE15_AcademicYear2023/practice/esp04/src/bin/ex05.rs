/*
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/

fn main() {
    let v1 = vec![1, 2, 3, 4];
    // let v1: [i32; 0] = [0; 0];
    let mut iter = v1.iter();

    if let Some(x) = iter.next() {
        println!("[[{x}]]");

        while let Some(x) = iter.next() {
            println!("[{x}]");
        }
    }
    else {
        println!("No values");
    }
}

fn main0() {
    let v1 = [1, 2, 3, 4];
    let mut iter = v1.iter();

    let mut x = iter.next().unwrap();
    println!("{}", x);

    x = iter.next().unwrap();
    println!("{}", x);

    x = iter.next().unwrap();
    println!("{}", x);

    x = iter.next().unwrap();
    println!("{}", x);

    x = iter.next().unwrap();
    println!("{}", x); // this won't run!
}
