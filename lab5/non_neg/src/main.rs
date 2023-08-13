fn extract_non_negatives(input_vec: &[f32]) -> Vec<f32> {
    let mut v1 = Vec::new();
    for _i in input_vec {
        if *_i > 0. {
            v1.push(*_i);
        }
    }
    return v1;
}

// 'a in Rust is a lifetime parameter that helps the compiler ensure that references remain valid and have the appropriate scope. 
fn extract_non_negatives_r<'a>(input_vec: &'a [f32], non_ng_list: &'a mut Vec<f32>) -> &'a mut Vec<f32> {
    if input_vec.is_empty() {
        return non_ng_list;
    }
    else {
        let f = input_vec[0];
        if f > 0. {
            non_ng_list.push(f);
        }
        extract_non_negatives_r(&input_vec[1..], non_ng_list);
        return non_ng_list;
    }
}


fn split_non_negative(input_vec: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut pos_v = Vec::new();
    let mut neg_v = Vec::new();
    for _i in input_vec {
        if *_i >= 0. {
            pos_v.push(*_i);
        }
        else {
            neg_v.push(*_i);
        }
    }
    return (pos_v, neg_v);
}

fn main() {
    let v = [0.8, -5.1, 1.6, -6.5, 10.5];
    let c1 = extract_non_negatives(&v);
    let c2 = split_non_negative(&v);

    let mut index: usize = 0;
    let mut non_ng_list = Vec::new();
    let c3 = extract_non_negatives_r(&v, &mut non_ng_list);
    println!("{:?}", c1);
    println!("{:?}", c2);
    println!("{:?}", c3);
}

#[test]
fn test_extract_non_negatives() {
    assert_eq!(extract_non_negatives(&[]), []);
    assert_eq!(
        extract_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
        [0.8, 1.6, 10.5]
    );
}

#[test]
fn test_split_non_negative() {
    assert_eq!(split_non_negative(&[]), (vec![], vec![]));
    assert_eq!(
        split_non_negative(&[0.8, -5.1, 1.6, -6.5, 10.5]),
        (
            vec![0.8, 1.6, 10.5],
            vec![-5.1, -6.5]
        )
    );
}



#[test]
fn test_extract_non_negatives_r() {
    let mut non_ng_list: Vec<f32> = Vec::new();
    let res = extract_non_negatives_r(&[0.8, -5.1, 1.6, -6.5, 10.5], &mut non_ng_list);

    assert_eq!(
        res.to_vec(), // because without to_vec() non_ng_list is &mut Vec<f32>
       vec![0.8, 1.6, 10.5]
    );
}
