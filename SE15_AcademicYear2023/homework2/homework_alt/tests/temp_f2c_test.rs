use assert_cmd::Command;

#[test]
fn without_fah(){
    let fah_arg = "";

    let expected = "Temperature at 0 Fahrenheit is -17.777779 Celsius\n";
    Command::cargo_bin("temperature_f2c").unwrap()
            .arg(fah_arg)
            .assert()
            .success().stdout(expected);
}

#[test]
fn with_fah(){
    let fah_arg = "30";

    let expected = "Temperature at 30 Fahrenheit is -1.1111112 Celsius\n";
    Command::cargo_bin("temperature_f2c").unwrap()
            .arg(fah_arg)
            .assert()
            .success().stdout(expected);
}
