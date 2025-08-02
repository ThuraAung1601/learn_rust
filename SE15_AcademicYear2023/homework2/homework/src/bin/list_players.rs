use clap::{Arg, App};

fn main() {
    let matches = App::new("Players")
        .version("0.1.0")
        .author("Thura Aung")
        .about("Listing the players")
        .arg(
            Arg::with_name("player1")
            .value_name("first player")
            .help("Name of first player")
            .required(false)
            .index(1)
        )
        .arg(
            Arg::with_name("player2")
            .value_name("second player")
            .help("Name of second player")
            .required(false)
            .index(2)
            
        ).setting(clap::AppSettings::AllowExternalSubcommands) // to discard coming commands after second argument index 2
        .get_matches();

    let player1 = matches.value_of("player1").unwrap_or("N/A");
    let player2 = matches.value_of("player2").unwrap_or("N/A");

    println!("Player 1: {}\nPlayer 2: {}", player1, player2);
}