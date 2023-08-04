fn doubles(v: &[i64]) -> Vec<i64> {
    v.iter().map(|x| x * 2).collect()
}

fn doubles_loop(v: &mut Vec<i64>) {
    for _i in 0..v.len() {
        v[_i] *= 2;
    }
}

fn doubles_recursion(v: &mut Vec<i64>, _i:usize) {
    if _i < v.len() {
        v[_i] *= 2;
        doubles_recursion(v, _i+1);
    }
}

#[test]
fn test_doubles() {
    let mut num = vec![1,2,3,-4];
    num = doubles(&mut num);
    let expected = vec![2,4,6,-8];
    assert_eq!(num,expected);
}

#[test]
fn test_loop() {
    let mut num = vec![1,2,3,-4];
    doubles_loop(&mut num);
    let expected = vec![2,4,6,-8];
    assert_eq!(num,expected);
}

#[test]
fn test_recursion() {
    let mut num = vec![1,2,3,-4];
    doubles_recursion(&mut num, 0);
    let expected = vec![2,4,6,-8];
    assert_eq!(num,expected);
}

