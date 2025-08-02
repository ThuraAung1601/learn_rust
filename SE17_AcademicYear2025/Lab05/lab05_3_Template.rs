use std::io;

// Recursive function to compute value at Pascal's Triangle position (row, col)
fn pascal(row: usize, col: usize) -> usize {
    if col == 0 || col == row {
        1
    } else {
        pascal(row - 1, col - 1) + pascal(row - 1, col)
    }
}

// Print a single row of Pascal's Triangle with alignment
fn print_pascal_row(n: usize, row: usize) {
    // Calculate padding for alignment
    let spaces = (n - row - 1) * 2;
    print!("{:width$}", "", width = spaces);

    // Print all values in this row
    for col in 0..=row {
        print!("{:>4}", pascal(row, col));
    }
    println!();
}

fn main() {
    let n: usize;

    // Input and validation loop
    loop {
        println!("Enter a number between 1 and 9:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 9 => {
                n = num;
                break;
            }
            _ => {
                println!("Invalid input. Please enter an integer from 1 to 9.");
            }
        }
    }

    // Generate and print Pascal’s Triangle
    println!("\nPascal's Triangle ({} rows):\n", n);
    for row in 0..n {
        print_pascal_row(n, row);
    }
}
