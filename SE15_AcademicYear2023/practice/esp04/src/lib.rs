// fn swap<T, U>(x: T, y: U) -> (U, T) { (y, x) }

// fn swap(x: f64, y: i64) -> (i64, f64) { (y, x) }
fn swap(x: i64, y: i64) -> (i64, i64) { (y, x) }

#[test]
fn test_sq() {
    assert_eq!(swap(1, 2), (2, 1));
    assert_eq!(swap(1.1, 2), (2, 1.1));
}

fn count_negative(v: &[i64]) -> usize {
    // v.iter().filter(|&&x| x < 0).count()
/*
    let mut c = 0;
    for i in 0..v.len() {
        if v[i] < 0 { c += 1 }
    }
    c
*/

    if v.len() < 1 { return 0 }
    if v[0] < 0 { return 1 + count_negative(&v[1..]) }
    count_negative(&v[1..])
}

#[test]
fn test_counting() {
    assert_eq!(count_negative(&[]), 0);
    assert_eq!(count_negative(&[1, 2, -3, 4, -6, 7]), 2);
}
