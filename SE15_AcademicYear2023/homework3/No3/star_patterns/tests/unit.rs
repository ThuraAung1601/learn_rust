// No 3.3
// How to run: cargo test
use assert_cmd::Command;

// ------------------------ Test for Star Pattern 1 -----------------------

#[test]
fn pattern1_without_size(){
    let output_size = "";

    let expected = "";
    Command::cargo_bin("star_pattern1").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size1(){
    let output_size = "1";

    let expected = "*\n";
    Command::cargo_bin("star_pattern1").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size2(){
    let output_size = "2";

    let expected = "*\n**\n*\n";
    Command::cargo_bin("star_pattern1").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size3(){
    let output_size = "3";

    let expected = "*\n**\n***\n**\n*\n";

    Command::cargo_bin("star_pattern1").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size4(){
    let output_size = "4";

    let expected = "*\n**\n***\n****\n***\n**\n*\n";

    Command::cargo_bin("star_pattern1").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size5(){
    let output_size = "5";

    let expected = "*\n**\n***\n****\n*****\n****\n***\n**\n*\n";

    Command::cargo_bin("star_pattern1").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

// ------------------------ Test for Star Pattern 2 -----------------------

#[test]
fn pattern2_without_size(){
    let output_size = "";

    let expected = "";
    Command::cargo_bin("star_pattern2").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size1(){
    let output_size = "1";

    let expected = "*\n";
    Command::cargo_bin("star_pattern2").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size2(){
    let output_size = "2";

    let expected = " *\n**\n *\n";
    Command::cargo_bin("star_pattern2").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size3(){
    let output_size = "3";

    let expected = "  *\n **\n***\n **\n  *\n";

    Command::cargo_bin("star_pattern2").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size4(){
    let output_size = "4";

    let expected = "   *\n  **\n ***\n****\n ***\n  **\n   *\n";

    Command::cargo_bin("star_pattern2").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size5(){
    let output_size = "5";

    let expected = "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n";

    Command::cargo_bin("star_pattern2").unwrap()
            .arg(output_size)
            .assert()
            .success().stdout(expected);
}
