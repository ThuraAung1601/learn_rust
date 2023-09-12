/*
No 1.1
Write a program that takes values from the command-line as a list of numbers and use
Vec::sort_by to sort numbers in ascending and descending order. Add an integration test
to verify the correctness of the program.

No 1.2
Modify the program in 1.1) to use loop and bubble sort instead of Vec::sort_by in order
to achieve the same result. Add an integration test to verify the correctness of the program.
*/

// No 1.2
fn bubble_sort(v: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut ascending = v.to_vec();
    let mut descending = v.to_vec();
    // ascending
    for i in 0..ascending.len() {
        for j in 0..ascending.len()-i-1 {
            // println!("{:?}\t{:?}", ascending[j], ascending[j+1]);
            if ascending[j] > ascending[j + 1] {
                let temp = ascending[j];
                ascending[j] = ascending[j+1];
                ascending[j+1] = temp;
            }
        }
    }
    // descending
    for i in 0..descending.len() {
        for j in 0..descending.len()-i-1 {
            // println!("{:?}\t{:?}", descending[j], descending[j+1]);
            if descending[j] < descending[j + 1] {
                let temp = descending[j];
                descending[j] = descending[j+1];
                descending[j+1] = temp;
            }
        }
    }
    return (ascending, descending);
}

fn main() {
    // let vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    let vec: Vec<f32> = std::env::args()
        .skip(1) 
        .map(|v| v.parse::<f32>().expect("Numbers only"))
        .collect();

    println!("Vector {:?}", vec);

    // No 1.1
    // Sort in ascending order
    let mut ascending_order = vec.clone();
    ascending_order.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Ascending: {:?}", ascending_order);
    // Sort in descending order
    let mut descending_order = vec.clone();
    descending_order.sort_by(|b, a| a.partial_cmp(b).unwrap());
    println!("Descending: {:?}", descending_order);

    // No 1.2
    // Sort using bubble sort
    let (ascending_order, descending_order) = bubble_sort(&vec);
    println!("Ascending using bubble sort: {:?}", ascending_order);
    println!("Descending using bubble sort: {:?}", descending_order);
}

#[test]
fn test_sort_by() {
    let vec = vec![4.0, 1.0, 2.6, 5.2, -3.1, -5.0, 0.0];
    let expected_a = vec![-5.0, -3.1, 0.0, 1.0, 2.6, 4.0, 5.2];
    let expected_d = vec![5.2, 4.0, 2.6, 1.0, 0.0, -3.1, -5.0];
    
    let mut ascending_order = vec.clone();
    ascending_order.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(ascending_order, expected_a);
    let mut descending_order = vec.clone();
    descending_order.sort_by(|b, a| a.partial_cmp(b).unwrap());
    assert_eq!(descending_order, expected_d);

    let (ascending_order_b, descending_order_b) = bubble_sort(&vec);
    assert_eq!(ascending_order_b, expected_a);
    assert_eq!(descending_order_b, expected_d);
}
