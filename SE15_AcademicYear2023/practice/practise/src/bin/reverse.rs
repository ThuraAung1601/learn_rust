fn reverse_r(v: &[i32]) -> Vec<i32> {
    let n = v.len();
    if n < 2 {
        return v.to_vec();
    }
    else {
        let (x1, x2) = (v[0], v[n-1]);
        let mut rest = Vec::new();
        rest.push(x2);
        rest.extend(reverse_r(&v[1..n-1]));
        rest.push(x1);
        return rest;
    }
}

fn main() {
    let v = [1, 2, 3];
    let rev = reverse_r(&v);
    println!("{:?}", rev);
}