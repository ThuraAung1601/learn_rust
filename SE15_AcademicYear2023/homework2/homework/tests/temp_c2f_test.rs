use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn without_cel(){
    Command::cargo_bin("temperature_c2f").unwrap()           
            .assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));

}

#[test]
fn with_cel(){
    let cel_arg = "20";
    let expected = "Temperature at 20 Celsius is 68 Fahrenheit\n";
    Command::cargo_bin("temperature_c2f").unwrap()
            .arg(cel_arg)
            .assert()
            .success().stdout(expected);
}