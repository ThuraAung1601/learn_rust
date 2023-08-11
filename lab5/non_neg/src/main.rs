fn extract_non_negatives(input_vec: &[f32]) -> Vec<f32> {
    let mut v1 = Vec::new();
    for _i in input_vec {
        if *_i > 0. {
            v1.push(*_i);
        }
    }
    return v1;
}

fn extract_non_negatives_r(input_list: &Vec<f32>, index : &mut usize, non_ng_list : &mut Vec<f32>) -> Vec<f32> {
    if input_list.len() ==0  {
        return  Vec::new();
    }

    let f = input_list[*index];
    if f >= 0.0 {
        non_ng_list.push(f);
    }

    *index +=1;
    if (*index < input_list.len()) {
        extract_non_negatives_r(input_list, index, non_ng_list);
    }

    // for f in input_list {
        
    // }

    non_ng_list.to_vec()
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

    let test_list = vec![1.0,-1.0,2.0,-2.0];
    let mut index: usize = 0;
    let mut non_ng_list: Vec<f32> = Vec::new();
    let c3 = extract_non_negatives_r(&test_list, &mut index , &mut non_ng_list);
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
fn test_extract_non_negatives_rec() {

    let test_list = vec![0.8, -5.1, 1.6, -6.5, 10.5];

    let mut index: usize = 0;

    let mut non_ng_list: Vec<f32> = Vec::new();


    let res = extract_non_negatives_r(&test_list, &mut index , &mut non_ng_list);

    assert_eq!(
        res,
        [0.8, 1.6, 10.5]
    );
}