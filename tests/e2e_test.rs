use std::process::Command;

#[test]
fn test_binary_search_success() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "--array", "1,2,3,4,5", "--target", "3"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Found 3 at index 2"));
}

#[test]
fn test_binary_search_not_found() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "--array", "1,2,3,4,5", "--target", "6"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Target 6 not found in array"));
}

#[test]
fn test_binary_search_empty_array() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "--array", "", "--target", "3"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Array is empty"));
}

#[test]
fn test_binary_search_generic_data_type() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "--array", "a,b,c,d,e", "--target", "c"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Found c at index 2"));
}
