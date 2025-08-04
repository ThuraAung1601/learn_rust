// Exercise: build a unique combinations (pairs)
// v: [1, 2, 3]
// result: [(1, 2), (1, 3), (2, 3)]

/*
[1, 2, 3] * [1, 2, 3] => [
    (1, 1), *(1, 2)*, *(1, 3)*,
    (2, 1), (2, 2), *(2, 3)*,
    (3, 1), (3, 2), (3, 3)
]
*/

// Exercise: reversing the values in the list
// reverse([]) => []
// reverse([x]) => [x]
// reverse([x, ..., y]) => [y, reverse(...), x]
fn reverse(v: &[i32]) -> Vec<i32> {
    let n = v.len();
    let i1 = 0;
    if n < 2 {
        return v.to_vec();
    }

    let (x1, x2) = (v[0], v[n - 1]);
    let mut res = Vec::new();
    res.push(x2);
    res.extend(reverse(&v[1..n - 1]));
    res.push(x1);
    res
}

fn reverse0(v: &[i32]) -> Vec<i32> {
    let mut it = v.iter();
    if let Some(&x1) = it.next() {
        println!("{x1}");
        if let Some(&x2) = it.next_back() {
            println!("{x2}");
            let mut v1 = Vec::new();
            v1.push(x2);

            //let v2: Vec<_> = reverse(&it.collect());
            //v1.extend(v2);
            v1.push(x1);
            return v1;
        }
    }
    Vec::new()
}

fn main() {
    println!("{:?}", reverse(&[2, 1, 3]));
}

// Exercise: calculate factorial value
// Factorial definition:
//     0! => 1
//     n! => n * (n - 1)!
fn factorial(n: i32) -> i32 {
    if n < 1 { return 1 }
    n * factorial(n - 1)
}

fn main0() {
    let nn = 6;
    for n in 0..nn {
        let x = factorial(n);
        println!("fac({n}) = {x}");
    }
}
