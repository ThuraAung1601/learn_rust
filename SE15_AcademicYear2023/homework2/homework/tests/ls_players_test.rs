use assert_cmd::Command;

#[test]
fn without_names() {

    let expected = "Player 1: N/A\nPlayer 2: N/A\n";
    Command::cargo_bin("list_players").unwrap()
                .assert()
                .success().stdout(expected);
}

#[test]
fn with_one_name() {

    let player1_arg = "Mike";
    let expected = "Player 1: Mike\nPlayer 2: N/A\n";

    Command::cargo_bin("list_players").unwrap()
        .arg(player1_arg)
        .assert()
        .success().stdout(expected);
}

#[test]
fn with_two_names() {

    let player1_arg= "Mike";
    let player2_arg = "Leo";
    let expected = "Player 1: Mike\nPlayer 2: Leo\n";

    Command::cargo_bin("list_players").unwrap()
        .args(vec![player1_arg, player2_arg])
        .assert()
        .success().stdout(expected);
}

#[test]
fn with_three_names() {

    let player1_arg= "Mike";
    let player2_arg = "Leo";
    let _player3_arg = "Ralph";
    let expected = "Player 1: Mike\nPlayer 2: Leo\n";

    Command::cargo_bin("list_players").unwrap()
        .args(vec![player1_arg, player2_arg])
        .assert()
        .success().stdout(expected);
}
