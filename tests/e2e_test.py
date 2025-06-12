import subprocess
import os
import pytest

# Test cases for the file-copy CLI

def test_copy_file_success(tmp_path):
    # Create a dummy source file
    source_file = tmp_path / "source.txt"
    source_file.write_text("This is a test file.")

    # Create a destination file
    dest_file = tmp_path / "destination.txt"

    # Run the file-copy CLI
    result = subprocess.run(["cargo", "run", str(source_file), str(dest_file)], capture_output=True, text=True)

    # Assert that the command was successful
    assert result.returncode == 0

    # Assert that the destination file exists and contains the same content as the source file
    assert dest_file.exists()
    assert dest_file.read_text() == "This is a test file."

def test_copy_file_to_existing_directory(tmp_path):
    # Create a dummy source file
    source_file = tmp_path / "source.txt"
    source_file.write_text("This is a test file.")

    # Create a destination directory
    dest_dir = tmp_path / "destination_dir"
    dest_dir.mkdir()

    # Run the file-copy CLI
    result = subprocess.run(["cargo", "run", str(source_file), str(dest_dir)], capture_output=True, text=True)

    # Assert that the command was successful
    assert result.returncode == 0

    # Assert that the file was copied into the destination directory
    copied_file = dest_dir / "source.txt"
    assert copied_file.exists()
    assert copied_file.read_text() == "This is a test file."


def test_copy_file_overwrite_existing_file(tmp_path):
    # Create a dummy source file
    source_file = tmp_path / "source.txt"
    source_file.write_text("This is the new content.")

    # Create a destination file with some initial content
    dest_file = tmp_path / "destination.txt"
    dest_file.write_text("This is the original content.")

    # Run the file-copy CLI
    result = subprocess.run(["cargo", "run", str(source_file), str(dest_file)], capture_output=True, text=True)

    # Assert that the command was successful
    assert result.returncode == 0

    # Assert that the destination file was overwritten with the content of the source file
    assert dest_file.exists()
    assert dest_file.read_text() == "This is the new content."


def test_copy_file_source_not_exists(tmp_path):
    # Create a non-existent source file path
    source_file = tmp_path / "non_existent_file.txt"

    # Create a destination file
    dest_file = tmp_path / "destination.txt"

    # Run the file-copy CLI
    result = subprocess.run(["cargo", "run", str(source_file), str(dest_file)], capture_output=True, text=True)

    # Assert that the command failed with a non-zero exit code
    assert result.returncode != 0

    # Assert that the error message is printed to stderr
    assert "No such file or directory" in result.stderr

    # Assert that the destination file does not exist
    assert not dest_file.exists()


def test_copy_file_destination_invalid(tmp_path):
    # Create a dummy source file
    source_file = tmp_path / "source.txt"
    source_file.write_text("This is a test file.")

    # Create an invalid destination path (e.g., a directory that doesn't exist and cannot be created)
    dest_dir = tmp_path / "non_existent_dir" / "destination.txt"

    # Run the file-copy CLI
    result = subprocess.run(["cargo", "run", str(source_file), str(dest_dir)], capture_output=True, text=True)

    # Assert that the command failed with a non-zero exit code
    assert result.returncode != 0

    # Assert that the error message is printed to stderr
    assert "No such file or directory" in result.stderr


def test_copy_file_verbose_flag(tmp_path):
    # Create a dummy source file
    source_file = tmp_path / "source.txt"
    source_file.write_text("This is a test file.")

    # Create a destination file
    dest_file = tmp_path / "destination.txt"

    # Run the file-copy CLI with the verbose flag
    result = subprocess.run(["cargo", "run", "--verbose", str(source_file), str(dest_file)], capture_output=True, text=True)

    # Assert that the command was successful
    assert result.returncode == 0

    # Assert that the verbose output is printed to stdout
    assert "Copying" in result.stdout
    assert "to" in result.stdout

