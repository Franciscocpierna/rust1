use std::process::Command;

#[test]
fn test_search_case_sensitive() {
    let output = Command::new("cargo")
        .args(&["run", "duct", "tests/file.txt"])
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);

    assert!(output_str.contains("safe, fast, productive."));
}

#[test]
fn test_search_case_insensitive() {
    let output = Command::new("cargo")
        .env("IGNORE_CASE", "1")
        .args(&["run", "rust", "tests/file.txt"])
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);

    assert!(output_str.contains("Rust:"));
    assert!(output_str.contains("Trust me."));
}