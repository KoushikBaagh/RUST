use std::process::Command;
use std::env;

#[test]
fn test_greet() {
    let bin = env::var("CARGO_BIN_EXE_rustr_app").expect("CARGO_BIN_EXE_rustr_app not set");
    let output = Command::new(bin).arg("greet").output().expect("failed to run process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, world!"));
}

#[test]
fn test_add() {
    let bin = env::var("CARGO_BIN_EXE_rustr_app").expect("CARGO_BIN_EXE_rustr_app not set");
    let output = Command::new(bin)
        .args(["add", "10", "-3"])
        .output()
        .expect("failed to run process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim() == "7");
}
