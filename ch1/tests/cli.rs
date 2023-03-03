use assert_cmd::Command;



#[test]
fn works_test() {
    assert!(true);
}

#[test]
fn runs_test() {
    let mut cmd = Command::cargo_bin("ch1").unwrap();
    cmd.assert().success().stdout("STDOUT!\n");
}

#[test]
fn true_ok_test() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();

}

#[test]
fn false_ok_test() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
