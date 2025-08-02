#[allow(dead_code)]
fn unpack_number_tuples_loop(tuples: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for (x, y) in tuples {
        first_numbers.push(*x);
        second_numbers.push(*y);
    }
    return (first_numbers, second_numbers);
}

// No 3.1
fn unpack_number_tuples(tuples: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    let mut iter = tuples.iter();
    while let Some((x,y)) = iter.next() {
        first_numbers.push(*x);
        second_numbers.push(*y);
    }
    return (first_numbers, second_numbers);
}

// No 3.2
fn unpack_number_tuples3(tuples: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();
    let mut iter = tuples.iter();
    while let Some((x, y, z)) = iter.next() {
        v1.push(*x);
        v2.push(*y);
        v3.push(*z);
    }
    return (v1, v2, v3);
}

fn main() {
    let v1 = [(1, 4), (3, 2), (2, 1)];
    let result = unpack_number_tuples(&v1);
    println!("{:?}", result);

    let v2 = [(1, 4, 5), (2, 2, 1)];
    let result2 = unpack_number_tuples3(&v2);
    println!("{:?}", result2);
}

// No 3.1
#[test]
fn test_unpack_number_tuples() {
    let input = [(1, 4), (3, 2), (2, 1)];
    let result = unpack_number_tuples(&input);
    assert_eq!(result, (vec![1, 3, 2], vec![4, 2, 1]));
}

// No 3.2
#[test]
fn test_unpack_number_tuples3() {
    let input = [(1, 4, 5), (2, 2, 1)];
    let result = unpack_number_tuples3(&input);
    assert_eq!(result, ((vec![1, 2], vec![4, 2], vec![5, 1])));
}

#[test]
fn test_unpack_number_tuples_empty_input() {
    let input: [(i32, i32); 0] = [];
    let result = unpack_number_tuples(&input);
    assert_eq!(result, (vec![], vec![]));
}

#[test]
fn test_unpack_number_tuples3_empty_input() {
    let input: [(i32, i32, i32); 0] = [];
    let result = unpack_number_tuples3(&input);
    assert_eq!(result, ((vec![], vec![], vec![])));
}
