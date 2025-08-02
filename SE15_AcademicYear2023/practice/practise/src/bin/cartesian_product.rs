fn build_cartesian_product(v1: &[i32], v2: &[i32]) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    for i in v1 {
        for j in v2 {
            v.push((*i, *j));
        }
    }
    return v;
}

fn build_cartesian_product_iter(v1: &[i32], v2: &[i32]) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    let mut v1_iter = v1.iter();
    while let Some(x) = v1_iter.next() {
        for i in v2 {
            v.push((*x, *i));
        }
    }
    return v;
}

fn main() {
    let v1 = [1, 2];
    let v2 = [21, 12, 34];
    let v = build_cartesian_product(&v1, &v2);
    println!("{:?}", v);
    let v_iter = build_cartesian_product_iter(&v1, &v2);
    println!("{:?}", v_iter);
}