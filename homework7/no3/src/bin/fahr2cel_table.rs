// No. 3.1
// Fahrenheit to Celsius conversion formula
// How to run: cargo run --bin fahr2cel_table > temp_result.html

fn fahr2cel(fahr: i32) -> f32 {
    (5.0/9.0)*(fahr as f32 - 32.0)
}

fn main() {

    let args:Vec<String> = std::env::args().collect();

    let start_arg = if args.len() < 2 {""} else {&args[1]};
    let end_arg = if args.len() < 3 {""} else {&args[2]};
    let step_arg = if args.len() < 4 {""} else {&args[3]};
    
    let mut start: i32 = start_arg.parse().unwrap_or(0);
    let mut end: i32 = end_arg.parse().unwrap_or(0);
    let mut step: usize = step_arg.parse().unwrap_or(0);

    if step == 0 {
        (start, end, step) = (1, 1, 1);
    }
    
    let mut html: String = "".to_string();
    
    // style
    // html.push_str("
    // <style>
    // table, td {
    // border: 1px solid #000000;
    // border-collapse: collapse;
    // }
    // </style>");

    // table start
    html.push_str("
    <table>
    ");
    
    // table head
    html.push_str("
    <tr>
    <th style=\"text-align:center\">Fahr</th>
    <th style=\"text-align:center\">Celcius</th>
    </tr>
    ");

    if start <= end {
        // Ref: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
        for fahr in (start..=end).step_by(step){
            let cel: f32 = fahr2cel(fahr);
            html.push_str(&format!("
            <tr>
            <td style=\"text-align:right\">{}</td>
            <td style=\"text-align:right\">{:.1}</td>
            </tr>
            ",fahr, cel))
        }
    }
    else {
        // Ref: https://users.rust-lang.org/t/reverse-for-loops/53856/2 
        for fahr in (end..=start).rev().step_by(step){
            let cel: f32 = fahr2cel(fahr);
            html.push_str(&format!("
            <tr>
            <td style=\"text-align:right\">{}</td>
            <td style=\"text-align:right\">{:.1}</td>
            </tr>
            ",fahr, cel))
        }
    }

    // table end
    html.push_str("
    </table>
    ");

    println!("{}", html);  
}