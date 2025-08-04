fn unique_combo(v: &[i32]) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        for j in i+1..v.len() {
            result.push((v[i], v[j]));
        }
    }
    return result;
}

fn main() {
    let v = [1, 2, 3];
    let result = unique_combo(&v);
    println!("{:?}", result);
}