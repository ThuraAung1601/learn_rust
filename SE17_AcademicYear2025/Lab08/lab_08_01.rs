// Calculates the factorial of a number recursively.
// Prints the value and memory address of the input 'n' at each step.
fn factorial(n: u64) -> u64 {
    println!(
        "  Value: {}, Memory Address: {:p}",
        n, // The current number
        &n  // A reference to the number to get its memory address
    );

    // Base case for the recursion: factorial of 0 is 1.
    if n == 0 {
        1
    } else {
        // Recursive step: n * factorial of (n-1).
        // A new stack frame is created for each call to factorial(n-1).
        n * factorial(n - 1)
    }
}

fn main() {
    // --- Task 1: Calculate factorial for a small number ---
    let num = 5; // let num = 1000;
    println!("Calculating factorial({})", num);
    let result = factorial(num);
    println!("Factorial result: {}", result);

    println!("\n--------------------------------------------------\n");
}
