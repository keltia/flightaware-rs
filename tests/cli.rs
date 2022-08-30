use assert_cmd::Command;

const BIN1: &str = "fa-export";

#[test]
fn test_export_empty_args() {
    let mut cmd = Command::cargo_bin(BIN1).unwrap();
    cmd.assert().success();
}

#[test]
fn test_export_help() {
    let mut cmd = Command::cargo_bin(BIN1).unwrap();
    cmd.arg("-h").assert().success();
}

#[test]
fn test_export_version() {
    let mut cmd = Command::cargo_bin(BIN1).unwrap();

    cmd.arg("-V").assert().success();
}
