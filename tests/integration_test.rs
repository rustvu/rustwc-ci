use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = assert_cmd::Command::cargo_bin("rustwc").unwrap();

    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

const TEXT: &str = "Hello, world!\nHello, world!\nHello, world!\n";
const LINES: usize = 3;
const WORDS: usize = 6;
const CHARS: usize = 42;

#[test]
fn test_all() {
    let test_file = assert_fs::NamedTempFile::new("test.txt").unwrap();
    test_file.write_str(TEXT).unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("rustwc").unwrap();

    let expected = format!("{:8}{:8}{:8}", LINES, WORDS, CHARS);
    cmd.arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_stdin() {
    let mut cmd = assert_cmd::Command::cargo_bin("rustwc").unwrap();

    let expected = format!("{:8}{:8}{:8}", LINES, WORDS, CHARS);
    cmd.write_stdin(TEXT)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_lines() {
    let test_file = assert_fs::NamedTempFile::new("test.txt").unwrap();
    test_file.write_str(TEXT).unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("rustwc").unwrap();

    let expected = format!("{:8}", LINES);
    cmd.arg("-l")
        .arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

// TODO: add more tests (-w, -c, multiple files, etc.)
