fn max(v: &[i32]) -> i32 {
    let mut max = v[0];
    for i in 0..v.len() {
        if max < v[i] {
            max = v[i];
        }
    }
    max
}

fn max2(v: &[i32]) -> i32 {
    let mut iter = v.iter();
    if let Some(mut max) = iter.next() {
        while let Some(x) = iter.next() {
            if max < x {
                max = x;
            }
        }
        return *max;
    }
    else {
        return 0;
    }
}

fn even_num(v: Vec<i32>) -> (Vec<i32>, usize)  {
    let even_ls = v.iter().filter(|&&s| s%2 == 0).cloned().collect();
    let even_count = v.iter().filter(|&&s| s%2 == 0).count();

    (even_ls, even_count)
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 100, 20, 11];
    println!("{}", max(&v1));
    println!("{}", max2(&v1));

    println!("{:?}", even_num(v1));

    let v2 = ["Tom", "James", "Mary"];
    let name_ls: Vec<String> = v2.iter().map(|s| s.to_string()).collect();
    println!("{:?}", name_ls);
}