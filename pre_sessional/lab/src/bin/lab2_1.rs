use std::io;
fn main() {
    // Read player names from standard input
    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("Enter name for Player 1:");
    io::stdin().read_line(&mut player1).expect("Failed to read input");
    println!("Enter name for Player 2:");
    io::stdin().read_line(&mut player2).expect("Failed to read input");

    // Trim the newline characters from the input
    player1 = player1.trim().to_string();
    player2 = player2.trim().to_string();

    // Create the vertical pattern
    let max_length = std::cmp::max(player1.len(), player2.len()) + 14;
    let border_line = "*".repeat(max_length);

    println!("Vertical Pattern:");
    println!("{}", border_line);
    println!("* Player 1: {:<max_length$}*", player1, max_length = max_length - 13);
    println!("* Player 2: {:<max_length$}*", player2, max_length = max_length - 13);
    println!("{}", border_line);

    // Create the horizontal pattern
    let horizontal_border_length = 13 + 13 + player1.len() + player2.len()+ 1;
    let horizontal_border_line = "*".repeat(horizontal_border_length);

    println!("\nHorizontal Pattern:");
    println!("{}", horizontal_border_line);
    print!("* Player 1: {} * Player 2: {} *", player1, player2);
    println!("{}", " ".repeat(horizontal_border_length - (13 + player1.len() + player2.len() + 2)));
    println!("{}", horizontal_border_line);
}