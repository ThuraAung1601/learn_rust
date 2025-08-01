/*
Exercises:
1. calculating min value from the number list
2. calculating max value from the number list
3. calculating (min, max) value from the number list
4. calculating sum value from the number list
5. calculating average value from the number list
6. calculating (sum, average) value from the number list
*/

// calculating min value from the number list
// Alt.: fn min_values(v: &[i32]) -> Option<i32>
fn min_values(v: &[i32]) -> i32 {
    let mut iter = v.iter();
    if let Some(mut min) = iter.next() {
        while let Some(x) = iter.next() {
            if x < min {
                min = x;
            }
        }
        *min
    }
    else {
        0
    }
}

// calculating (min, max) value from the number list
fn min_max_values(v: &[i32]) -> (i32, i32) {
    let mut iter = v.iter();
    if let Some(mut min) = iter.next() {
        let mut max = min;
        while let Some(x) = iter.next() {
            if x < min {
                min = x;
            }
            if max < x {
                max = x;
            }
        }
        (*min, *max)
    }
    else {
        (0, 0)
    }
}

fn main() {
    let v = [2, 1, 3];
    let min = min_values(&v);
    println!("{min}");

    let (min, max) = min_max_values(&v);
    println!("MIN: {min}, MAX: {max}");
}

