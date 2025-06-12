import subprocess
import os
import pytest

# Test data
SOURCE_FILE = "test_source.txt"
DEST_FILE = "test_dest.txt"

# Create test files if they don't exist
if not os.path.exists(SOURCE_FILE):
    with open(SOURCE_FILE, "w") as f:
        f.write("This is a test source file.")

if os.path.exists(DEST_FILE):
    os.remove(DEST_FILE)


def run_cli(source, destination):
    try:
        result = subprocess.run(["cargo", "run", "--", source, destination], capture_output=True, text=True, check=True)
        return result.stdout.strip(), result.stderr.strip()
    except subprocess.CalledProcessError as e:
        return e.stdout.strip(), e.stderr.strip()



def test_file_copy_success():
    stdout, stderr = run_cli(SOURCE_FILE, DEST_FILE)
    assert os.path.exists(DEST_FILE)
    with open(DEST_FILE, "r") as f:
        assert f.read() == "This is a test source file."
    assert "" == stderr, f"Expected no error, but got: {stderr}"


def test_file_overwrite():
    # Create destination file first
    with open(DEST_FILE, "w") as f:
        f.write("Original content")

    stdout, stderr = run_cli(SOURCE_FILE, DEST_FILE)
    assert os.path.exists(DEST_FILE)
    with open(DEST_FILE, "r") as f:
        assert f.read() == "This is a test source file."
    assert "" == stderr, f"Expected no error, but got: {stderr}"


def test_source_file_not_found():
    stdout, stderr = run_cli("non_existent_file.txt", DEST_FILE)
    assert "No such file or directory" in stderr or "FileNotFound" in stderr
    assert not os.path.exists(DEST_FILE)


def test_destination_permission_error(tmpdir):
    # Create a read-only directory
    dest_dir = tmpdir.mkdir("readonly")
    dest_file = dest_dir.join("test_dest.txt")
    os.chmod(str(dest_dir), 0o555)  # Read and execute permissions for all

    stdout, stderr = run_cli(SOURCE_FILE, str(dest_file))

    assert "Permission denied" in stderr or "Access is denied" in stderr
    assert not os.path.exists(dest_file)

    # Restore permissions
    os.chmod(str(dest_dir), 0o777)


