fn min_value(v: &[i32]) -> i32 {
    let mut min = v[0];
    for i in 0..v.len() {
        if min > v[i] {
            min = v[i];
        }
        else {
            continue;
        }
    }
    return min;
}

fn max_value(v: &[i32]) -> i32 {
    let mut max = v[0];
    for i in 0..v.len() {
        if max < v[i] {
            max = v[i];
        }
        else {
            continue;
        }
    }
    return max;
}

// Using iterators
fn min_value_iter(v: &[i32]) -> i32 {
    let mut iter = v.iter();
    if let Some(mut min) = iter.next() {
        while let Some(x) = iter.next() {
            if x < min {
                min = x;
            }
        }
        return *min;
    }
    else {
        return 0;
    }
}

fn max_value_iter(v:&[i32]) -> i32 {
    let mut iter = v.iter();
    if let Some(mut max) = iter.next() {
        while let Some(x) = iter.next() {
            if max <= x {
                max = x;
            }
        }
        return *max;
    }
    else {
        return 0;
    }
}

fn min_max_value(v: &[i32]) -> (i32, i32) {
    let (mut min, mut max) = (v[0], v[0]);
    for i in 0..v.len() {
        if min > v[i] {
            min = v[i];
        }
        if max < v[i] {
            max = v[i];
        }
        else {
            continue;
        }
    }
    return (min, max);
}

fn min_max_value_iter(v: &[i32]) -> (i32, i32) {
    let mut min_iter = v.iter();
    let mut max_iter = v.iter();
    if let (Some(mut min), Some(mut max)) = (min_iter.next(), max_iter.next()) {
        while let (Some(mut x), Some(mut y)) = (min_iter.next(), max_iter.next()) {
            if min > x {
                min = x;
            }
            if max < y {
                max = y;
            }
        }
        return (*min, *max);
    }
    else {
        return (0, 0);
    }
}

fn sum(v: &[i32]) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += *i;
    }
    return sum;
}

fn sum_iter(v: &[i32]) -> i32 {
    let mut iter = v.iter();
    let mut sum = 0;
    while let Some(x) = iter.next() {
            sum += *x;
        }
    return sum;
}

fn avg(v: &[i32]) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += *i; 
    }
    let len = v.len() as i32;
    let avg = sum/len;
    return avg;
}

fn avg_iter(v: &[i32]) -> i32 {
    let mut iter = v.iter();
    let mut sum = 0;
    while let Some(x) = iter.next() {
        sum += *x;
    }
    let len = v.len() as i32;
    let avg = sum/len;
    return avg;
}

fn sum_avg(v: &[i32]) -> (i32, i32) {
    let mut sum = 0;
    for i in v {
        sum += *i;
    }
    let len = v.len() as i32;
    let avg = sum/len;
    return (sum, avg);
}

fn sum_avg_iter(v: &[i32]) -> (i32, i32) {
    let mut iter = v.iter();
    let mut sum = 0;
    while let Some(x) = iter.next() {
        sum += *x;
    }
    let len = v.len() as i32;
    let avg = sum/len;
    return (sum, avg);
}

fn main() {
    let num = [2,3,4,5,6,1,100,12];
    
    let min = min_value(&num);
    let max = max_value(&num);
    
    let min_iter = min_value_iter(&num);
    let max_iter = max_value_iter(&num);

    println!("Num: {:?}", num);

    println!("Min: {}", min);
    println!("Max: {}", max);
    
    println!("Min: {}", min_iter);
    println!("Min: {}", max_iter);

    let min_max = min_max_value(&num);
    println!("min, max = {:?}", min_max);
    let min_max_iter = min_max_value_iter(&num);
    println!("min, max iter = {:?}", min_max_iter);

    let sum = sum(&num);
    println!("Sum = {}", sum);

    let sum_iter = sum_iter(&num);
    println!("Sum iter = {}", sum_iter);

    let avg = avg(&num);
    println!("Average = {}", avg);

    let avg_iter = avg_iter(&num);
    println!("Average iter = {}", avg_iter);

    let sum_avg = sum_avg(&num);
    println!("Sum, average is {:?}", sum_avg);

    let sum_avg_iter = sum_avg_iter(&num);
    println!("Sum, average is {:?}", sum_avg_iter);
}
