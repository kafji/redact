use assert_cmd::Command;
use std::{fs, io::Write};
use tempfile;

#[test]
fn redact_text() {
    let output = Command::cargo_bin("redact")
        .unwrap()
        .arg("Hello world!")
        .env("REDACT_KEYWORDS", r#"["hello", "world"]"#)
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!("█████ █████!\n", stdout);
}

#[test]
fn redact_file() {
    let mut input_file = tempfile::NamedTempFile::new().unwrap();
    input_file.write("Hello world!".as_bytes()).unwrap();

    let output = Command::cargo_bin("redact")
        .unwrap()
        .args(&["-f", &*input_file.path().to_string_lossy()])
        .env("REDACT_KEYWORDS", r#"["hello"]"#)
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!("█████ world!\n", stdout);
}

#[test]
fn redact_rust_compilation_error() {
    let output = Command::cargo_bin("redact")
        .unwrap()
        .args(&["-f", "tests/assets/rust_compilation_error/to_redact.txt"])
        .env(
            "REDACT_KEYWORDS",
            include_str!("assets/rust_compilation_error/keywords").trim(),
        )
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut expected =
        fs::read_to_string("tests/assets/rust_compilation_error/redacted.txt").unwrap();
    expected.push_str("\n");
    assert_eq!(expected, stdout);
}
