use assert_cmd::Command;

#[test]
fn without_cel(){
    let cel_arg = "";
    let expected = "Temperature at 0 Celsius is 32 Fahrenheit\n";
    Command::cargo_bin("temperature_c2f").unwrap()
            .arg(cel_arg)
            .assert()
            .success().stdout(expected);
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
