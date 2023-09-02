// No 1.1
fn min_max_avg(v: &[i32]) -> (i32, i32, f32) {
    if v.is_empty() {
        return (0,0,0.);
    }
    else {
        let mut sum = 0.0;
        let (mut min, mut max) = (v[0], v[0]);
        for i in 0..v.len() {
            if min > v[i] {
                min = v[i];
            }
            if max < v[i] {
                max = v[i];
            }
            sum += v[i] as f32;
        }
        let len = v.len() as f32;
        let avg = sum/len;
        return (min, max, avg);
    }
}

// No 1.2
fn partial_sum(v: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for _i in 0..v.len() {
        if _i == 0 {
            result.push(v[_i]);
        }
        else {
        let val = v[_i]+result[_i-1];
        result.push(val);
        }
    }
    return result;
}

fn main() {
    let v1 = [2,3,4,5,6,1,100,12];
    let result1 = min_max_avg(&v1);
    println!("{:?}", result1);

    let v2 = [2, 11, 3, 4, 7];
    let result2 = partial_sum(&v2);
    println!("{:?}", result2);
}

// No 1.1
#[test]
fn test_min_max_avg() {
    let v = [2,3,4,5,6,1,100,12];
    let result = (1, 100, 16.625);
    assert_eq!(min_max_avg(&v), result);
}

// No 1.2
#[test]
fn test_partial_sum() {
    let v = [2, 11, 3, 4, 7];
    let result = vec![2, 13, 16, 20, 27];
    assert_eq!(partial_sum(&v), result);
}

#[test]
fn test_empty_input_min_max_avg() {
    let v: [i32; 0] = [];
    let result = (0,0,0.0);
    assert_eq!(min_max_avg(&v), result);
}

#[test]
fn test_empty_input_partial_sum() {
    let v: [i32; 0] = [];
    let result = Vec::new();
    assert_eq!(partial_sum(&v), result);
}