use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn without_radius(){
    Command::cargo_bin("circle").unwrap()
            .assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn with_radius(){
    let radius_arg = "7";

    let expected = "The area of the circle with radius 7 is 153.9384\n";
    Command::cargo_bin("circle").unwrap()
            .arg(radius_arg)
            .assert()
            .success().stdout(expected);
}
