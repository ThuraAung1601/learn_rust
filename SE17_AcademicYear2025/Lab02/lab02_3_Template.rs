use std::io;

fn main() {
    println!("--- Number Guessing Game ---");

    let secret_number: u32 = 42;

    // Track how many attempts were made.

    let mut attempts = 0;

    loop {
        println!("\nGuess the number (1-100):");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If parsing worked, `num` is the number.
            Err(_) => {
                // If it failed (e.g., user typed "abc"), print a message
                // and `continue` to the next iteration of the loop.
                println!("Please type a number!");
                continue;
            }
        };

        // Increment the attempt counter on each valid guess.
        attempts = attempts + 1;

        // 4) On each guess, use if/else to compare.
        if guess < secret_number {
            println!("Too low.");
        } else if guess > secret_number {
            println!("Too high.");
        } else {
            // If the guess is not lower or higher, it must be correct.
            println!("Correct!");
            println!("You guessed it in {} tries.", attempts);
            // `break` exits the loop, and the program ends.
            break;
        }
    }
}
