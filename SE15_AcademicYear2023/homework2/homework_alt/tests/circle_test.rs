use assert_cmd::Command;

#[test]
fn without_radius(){
    let radius_arg = "";

    let expected = "The area of the circle with radius 0 is 0\n";
    Command::cargo_bin("circle").unwrap()
            .arg(radius_arg)
            .assert()
            .success().stdout(expected);
}

#[test]
fn with_radius(){
    let radius_arg = "3";

    let expected = "The area of the circle with radius 3 is 28.274399\n";
    Command::cargo_bin("circle").unwrap()
            .arg(radius_arg)
            .assert()
            .success().stdout(expected);
}
