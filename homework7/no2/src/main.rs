/*
No. 1.1
Write a program that takes values from the command-line as a list of the pair (x, y) for
the point coordinate (for example, cargo run — 1 5 2 7 3 will make the point list 
[(1.,5.), (2., 7.)] (discarding 3 )) and use Vec::sort_by to sort points by x then y in
ascending and descending order. Add an integration test to verify the correctness of the
program.

No. 1.2
Modify the program to use loop and bubble sort instead of Vec::sort_by in order
to achieve the same result. Add an integration test to verify the correctness of the program.
*/

fn point(mut pts: Vec<f32>) -> Vec<(f32, f32)> {
    if pts.len()%2 != 0 {
        pts.remove(pts.len()-1);
    }
    // println!("{:?}", vec);
    let mut result = Vec::new();
    for i in 0..pts.len() {
        if i == 0 || i%2 == 0 {
            result.push((pts[i], pts[i+1]));
        }
    }
    return result;
}

// No 1.2
fn bubble_sort(pts: Vec<(f32,f32)>) -> (Vec<(f32,f32)>, Vec<(f32,f32)>) {
    let mut ascending = pts.clone();
    let mut descending = pts.clone();

    for _i in 0..ascending.len() {
        // println!("{:?}", pts[_i]);
        for _j in 0..ascending.len()-_i-1 {
            if ascending[_j].0 > ascending[_j+1].0 
                || (ascending[_j].0 == ascending[_j+1].0 && ascending[_j].1 > ascending[_j+1].1) {
                    let temp = ascending[_j];
                    ascending[_j] = ascending[_j+1];
                    ascending[_j+1] = temp;
                }
            }
        }

    for _i in 0..descending.len() {
        // println!("{:?}", pts[_i]);
        for _j in 0..descending.len()-_i-1 {
            if descending[_j].0 < descending[_j+1].0 
                || (ascending[_j].0 == ascending[_j+1].0 && descending[_j].1 < descending[_j+1].1) {
                    let temp = descending[_j];
                    descending[_j] = descending[_j+1];
                    descending[_j+1] = temp;
                }
            }
        }

    return (ascending, descending);
}

fn main() {
    let vec: Vec<f32> = std::env::args()
                .skip(1)
                .map(|v| v.parse::<f32>().expect("Number only"))
                .collect();
    
    let pts = point(vec);
    println!("Points: {:?}", pts);

    // No 1.1
    let mut ascending_order = pts.clone();
    let mut descending_order = pts.clone();
    ascending_order.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    descending_order.sort_by(|(x1, y1), (x0, y0)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    println!("Ascending: {:?}", ascending_order);
    println!("Descending: {:?}", descending_order);

    println!();

    // No 1.2
    let (ascending_order_b, descending_order_b) = bubble_sort(pts);
    println!("Ascending with Bubble sort: {:?}", ascending_order_b);
    println!("Descending with Bubble sort: {:?}", descending_order_b);
}

#[test]
fn test_sort_point() {
    let input = vec![1.0, 3.0, 5.0, 2.0, 1.0, 1.0, 0.0, 9.0, -5.0, 10.0, 3.0];

    let points = point(input);
    let expected = vec![(1.0, 3.0), (5.0, 2.0), (1.0, 1.0), (0.0, 9.0), (-5.0, 10.0)];
    assert_eq!(points, expected);

    let mut ascending_order = points.clone();
    ascending_order.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    let expected_a = vec![(-5.0, 10.0), (0.0, 9.0), (1.0, 1.0), (1.0, 3.0), (5.0, 2.0)];
    assert_eq!(ascending_order, expected_a);

    let mut descending_order = points.clone();
    descending_order.sort_by(|(x1, y1), (x0, y0)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    let expected_d = vec![(5.0, 2.0), (1.0, 3.0), (1.0, 1.0), (0.0, 9.0), (-5.0, 10.0)];
    assert_eq!(descending_order, expected_d);

    let (ascending_order_b, descending_order_b) = bubble_sort(points);
    let expected_a = vec![(-5.0, 10.0), (0.0, 9.0), (1.0, 1.0), (1.0, 3.0), (5.0, 2.0)];
    assert_eq!(ascending_order_b, expected_a);
    let expected_d = vec![(5.0, 2.0), (1.0, 3.0), (1.0, 1.0), (0.0, 9.0), (-5.0, 10.0)];
    assert_eq!(descending_order_b, expected_d);
}