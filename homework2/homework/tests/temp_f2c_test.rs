use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn without_fah(){
    Command::cargo_bin("temperature_f2c").unwrap()
            .assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn with_fah(){
    let fah_arg = "30";

    let expected = "Temperature at 30 Fahrenheit is -1.1111111111111112 Celsius\n";
    Command::cargo_bin("temperature_f2c").unwrap()
            .arg(fah_arg)
            .assert()
            .success().stdout(expected);
}