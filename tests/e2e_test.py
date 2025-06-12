import subprocess
import os
import pytest

# Test cases

def test_file_copy_success():
    # Create a dummy source file
    source_file = "test_source.txt"
    dest_file = "test_dest.txt"
    with open(source_file, "w") as f:
        f.write("This is a test file.")

    # Run the file copy command
    result = subprocess.run(["cargo", "run", "--", source_file, dest_file], capture_output=True, text=True)

    # Assert that the command was successful
    assert result.returncode == 0
    assert os.path.exists(dest_file)

    # Clean up
    os.remove(source_file)
    os.remove(dest_file)


def test_file_copy_source_not_found():
    # Define source and destination files
    source_file = "non_existent_source.txt"
    dest_file = "test_dest.txt"

    # Run the file copy command
    result = subprocess.run(["cargo", "run", "--", source_file, dest_file], capture_output=True, text=True)

    # Assert that the command failed with the correct error message
    assert result.returncode != 0
    assert "No such file or directory" in result.stderr


def test_file_copy_destination_exists():
    # Create a dummy source file
    source_file = "test_source.txt"
    dest_file = "test_dest.txt"
    with open(source_file, "w") as f:
        f.write("This is a test file.")
    with open(dest_file, "w") as f:
        f.write("This is an existing destination file.")

    # Run the file copy command
    result = subprocess.run(["cargo", "run", "--", source_file, dest_file], capture_output=True, text=True)

    # Assert that the command was successful
    assert result.returncode == 0
    assert os.path.exists(dest_file)

    # Clean up
    os.remove(source_file)
    os.remove(dest_file)
