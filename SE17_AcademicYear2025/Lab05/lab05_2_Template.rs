use std::io;

// Recursive version without error handling
fn sum_of_digits(n: u32) -> u32 {
    if n < 10 {
        n
    } else {
        (n % 10) + sum_of_digits(n / 10)
    }
}

// Version with Result to handle negative input
fn sum_of_digits_checked(n: i32) -> Result<u32, String> {
    if n < 0 {
        Err(String::from("Negative number not allowed"))
    } else {
        Ok(sum_of_digits(n as u32))
    }
}

fn main() {
    println!("Enter a non-negative integer:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let number: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    match sum_of_digits_checked(number) {
        Ok(result) => println!("Sum of digits = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
