import subprocess
import os
import pytest

# Test cases

def test_successful_copy(tmp_path):
    source_file = tmp_path / "source.txt"
    dest_file = tmp_path / "dest.txt"
    source_file.write_text("This is the source file.")

    result = subprocess.run(["cargo", "run", "--", str(source_file), str(dest_file)], capture_output=True, text=True)
    assert result.returncode == 0
    assert dest_file.read_text() == "This is the source file."


def test_overwrite_existing_file(tmp_path):
    source_file = tmp_path / "source.txt"
    dest_file = tmp_path / "dest.txt"
    source_file.write_text("This is the new content.")
    dest_file.write_text("This is the original content.")

    result = subprocess.run(["cargo", "run", "--", str(source_file), str(dest_file)], capture_output=True, text=True)
    assert result.returncode == 0
    assert dest_file.read_text() == "This is the new content."


def test_file_not_found_error(tmp_path):
    source_file = tmp_path / "nonexistent.txt"
    dest_file = tmp_path / "dest.txt"

    result = subprocess.run(["cargo", "run", "--", str(source_file), str(dest_file)], capture_output=True, text=True)
    assert result.returncode != 0
    assert "No such file or directory" in result.stderr


def test_permission_denied_error(tmp_path, monkeypatch):
    source_file = tmp_path / "source.txt"
    dest_file = tmp_path / "dest.txt"
    source_file.write_text("This is the source file.")
    
    # Make the destination directory read-only
    os.chmod(tmp_path, 0o555)
    
    result = subprocess.run(["cargo", "run", "--", str(source_file), str(dest_file)], capture_output=True, text=True)
    assert result.returncode != 0
    assert "Permission denied" in result.stderr
    
    # Restore permissions
    os.chmod(tmp_path, 0o777)
