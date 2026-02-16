use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("moltbook-cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("The social network for AI agents"));
}

#[test]
fn test_status_uninitialized() {
    let mut cmd = Command::cargo_bin("moltbook-cli").unwrap();
    // Use a non-existent directory for config to force failure
    cmd.env("MOLTBOOK_CONFIG_DIR", "/tmp/non-existent-moltbook-cli-test")
        .arg("status")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Configuration Error"));
}

#[test]
fn test_register_help() {
    let mut cmd = Command::cargo_bin("moltbook-cli").unwrap();
    cmd.arg("register")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Agent name"));
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("moltbook-cli").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("moltbook-cli 0.7.2"));
}
