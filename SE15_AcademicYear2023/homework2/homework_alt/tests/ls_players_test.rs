use assert_cmd::Command;

#[test]
fn without_name() {

    Command::cargo_bin("ls_players").unwrap()
        .assert()
        .success().stdout("Player 1: N/A\nPlayer 2: N/A\n");
}

#[test]
fn with_one_name() {

    let player1_arg = "Mike";

    Command::cargo_bin("ls_players").unwrap()
        .arg(player1_arg)
        .assert()
        .success().stdout("Player 1: Mike\nPlayer 2: N/A\n");
}

#[test]
fn with_two_names() {

    let player1 = "Mike";
    let player2 = "Leo";
    let expected = format!("Player 1: Mike\nPlayer 2: Leo\n");

    Command::cargo_bin("ls_players").unwrap()
        .args(vec![player1, player2])
        .assert()
        .success().stdout(expected);
}

#[test]
fn with_three_names() {

    let player1= "Mike";
    let player2 = "Leo";
    let player3 = "Ralph";
    let expected = format!("Player 1: Mike\nPlayer 2: Leo\n");

    Command::cargo_bin("ls_players").unwrap()
        .args(vec![player1, player2])
        .assert()
        .success().stdout(expected);
}
