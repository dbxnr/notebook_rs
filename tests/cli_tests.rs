use snapbox::cmd::cargo_bin;
use snapbox::cmd::Command;

#[test]
fn test_read_first() {
    Command::new(cargo_bin!("nb"))
        .arg("-c")
        .arg("data/test_config.toml")
        .arg("-r")
        .arg("0")
        .assert()
        .stdout_eq_path("tests/cmd/test_read_first.stdout");
}

#[test]
fn test_read_last() {
    Command::new(cargo_bin!("nb"))
        .arg("-c")
        .arg("data/test_config.toml")
        .arg("-r")
        .arg("3")
        .assert()
        .stdout_eq_path("tests/cmd/test_read_last.stdout");
}

#[test]
fn test_list_all() {
    Command::new(cargo_bin!("nb"))
        .arg("-c")
        .arg("data/test_config.toml")
        .arg("-l")
        .assert()
        .stdout_eq_path("tests/cmd/test_list_all.stdout");
}

#[test]
fn test_help() {
    Command::new(cargo_bin!("nb"))
        .arg("-h")
        .assert()
        .stdout_eq_path("tests/cmd/test_help.stdout");
}
