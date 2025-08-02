// No. 3.2
// Format x, x^2 and x^3
// How to run: cargo run --bin number_table <start> <end> <step> > num_table.html

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
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
    // table start
    html.push_str("
    <table>
    ");
    
    // table head
    html.push_str("
    <tr>
    <th style=\"text-align:right\">x</th>
    <th style=\"text-align:right\">x<sup>2</sup></th>
    <th style=\"text-align:right\">x<sup>3</sup></th>
    </tr>
    ");

    if start <= end {
        for i in (start..=end).step_by(step){
            // println!("{}\t{}\t{}", i, i.pow(2), i.pow(3));
            html.push_str(&format!("
            <tr>
            <td style=\"text-align:right\">{}</td>
            <td style=\"text-align:right\">{}</td>
            <td style=\"text-align:right\">{}</td>
            </tr>
            ",i, i.pow(2), i.pow(3)))
        }
    }
    else {
        for i in (end..=start).rev().step_by(step){
        // println!("{}\t{}\t{}", i, i.pow(2), i.pow(3));
            html.push_str(&format!("
            <tr>
            <td style=\"text-align:right\">{}</td>
            <td style=\"text-align:right\">{}</td>
            <td style=\"text-align:right\">{}</td>
            </tr>
            ",i, i.pow(2), i.pow(3)))
        }
    }

    // table end
    html.push_str("
    </table>
    ");

    println!("{}", html);
}
