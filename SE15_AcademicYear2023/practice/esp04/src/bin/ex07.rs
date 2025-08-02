// Exercise: build a cartesian product
// v1: [1, 2, 3]
// v2: [21, 12]
// result: [(1, 21), (1, 12), (2, 21), (2, 12), (3, 21), (3, 12)]
fn build_cartesian(v1: &[i32], v2: &[i32]) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for &x1 in v1 {
        for &x2 in v2 {
            res.push((x1, x2));
        }
    }
    res
}

fn main() {
    let v1 = [1, 2, 3];
    let v2 = [21, 12];
    let p = build_cartesian(&v1, &v2);
    println!("{:?}", p);

    let mut ix = p.iter();
    while let Some((x, y)) = ix.next() {
        println!("*({x}, {y})*");
    }
}

fn main0() {
    let v1 = [2, 1, 3];
    let v2: Vec<_> = v1.iter().map(|x| x * x).collect();
    println!("{:?}", v2);
}
