// fn bubble_sort(v: &[i32]) -> Vec<i32> {
//     let mut v1 = v.to_vec();
//     for i in 0..v1.len() {
//         for j in 0..v1.len()-i-1 {
//             if v1[j] > v1[j+1] {
//                 let temp = v[j];
//                 v1[j] = v1[j+1];
//                 v1[j+1] = temp;
//             }
//         }
//     }
//     v1
// }

// fn main() {
//     let a = 31; let b = 5;
//     println!("{:?}", a.partial_cmp(&b));

//     let v1 = vec![1, 2, 3, 4, 6, 4, 2, 3, 1, 100, 2, -1, 2, 0, 3, 2, 1230, 20];
//     let mut v2 = v1.clone();
//     v2.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     println!("{:?}", v2);

//     println!("{:?}", bubble_sort(&v2));
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Put points");
        std::process::exit(1);
    }

    let mut points: Vec<f32> = args.iter().skip(1).map(|s| s.parse::<f32>().unwrap_or(0.0)).collect();
    println!("{:?}", points);

    if points.len() % 2 != 0 {
        points.remove(points.len() - 1);
    }
    println!("{:?}", points);

    let mut pt_ls = Vec::new();
    for i in 0..points.len() {
        if i == 0 || i%2 == 0 {
            pt_ls.push((points[i], points[i+1]));
        }
    }
    println!("{:?}", pt_ls);

    let mut sorted = pt_ls.clone();
    sorted.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    println!("{:?}", sorted);
}