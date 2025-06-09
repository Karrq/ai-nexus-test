use std::process::Command;

#[test]
fn test_binary_search_cli() {
    // Build the project
    let build_status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build project");

    assert!(build_status.success(), "Cargo build failed");

    // Run the binary with a sorted list and a target
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("[2, 5, 7, 8, 11, 12]")
        .arg("11")
        .output()
        .expect("Failed to run binary");

    // Check the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("4"), "Output does not contain expected index");
}
