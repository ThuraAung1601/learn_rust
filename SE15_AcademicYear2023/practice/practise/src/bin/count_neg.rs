fn count_neg(v: &[i32]) -> usize {
    let count = v.iter().filter(|&&x| x < 0).count();
    return count;
}

fn count_neg_loop(v: &[i32]) -> i32 {
    let mut count = 0;
    for i in v{
        if *i < 0 {
            count += 1;
        }
    }
    return count;
}

fn count_neg_r(v: &[i32], count: i32) -> i32 {
    if v.len() == 0 {
        return count;
    }
    else {
        if v[0] < 0 {
            count_neg_r(&v[1..], count+1)
        }
        else {
            count_neg_r(&v[1..], count)
        }
    }
}

fn collect_neg(v: &[i32]) -> Vec<&i32> {
    let neg: Vec<_> = v.iter().filter(|&&x| x < 0).collect();
    return neg.to_vec();
}

fn collect_neg_loop(v: &[i32]) -> Vec<i32> {
    let mut neg = Vec::new();
    for i in v {
        if *i < 0 {
            neg.push(*i);
        }
        else {
            continue;
        }
    }
    return neg;
}

fn collect_neg_r(v: &[i32], neg: &mut Vec<i32>) -> Vec<i32> {
    if v.len() == 0 {
        return neg.to_vec();
    }
    else {
        if v[0] < 0 {
            neg.push(v[0]);
            collect_neg_r(&v[1..], neg)
        }
        else {
            collect_neg_r(&v[1..], neg)
        }
    }

}

fn main(){
    let v = [1, -2, -3, 4, 5, -6];

    println!("{}", count_neg(&v));
    println!("{}", count_neg_loop(&v));
    println!("{}", count_neg_r(&v, 0));

    println!("{:?}", collect_neg(&v));
    println!("{:?}", collect_neg_loop(&v));
    println!("{:?}", collect_neg_r(&v, &mut Vec::new()));
}