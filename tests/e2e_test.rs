use std::process::Command;

#[test]
fn test_fibonacci_cli() {
    // Compile the fibonacci_sequence application
    let mut cmd = Command::new("cargo");
    cmd.arg("build").arg("--release");
    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    // Execute the compiled binary
    let mut cmd = Command::new("target/release/fibonacci_sequence");
    let output = cmd.output().expect("Failed to execute command");
    assert!(output.status.success());

    // Verify the output (replace with your expected output)
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Fibonacci Sequence"));
}
