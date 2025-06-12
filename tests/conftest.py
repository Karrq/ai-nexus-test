import pytest
import os

# This file can be used to store pytest configurations or fixtures
# that are shared across multiple test files.

# Example: Define a fixture to set up and tear down a test environment
@pytest.fixture(scope="session", autouse=True)
def test_environment():
    # Setup code (e.g., create directories, files, etc.)
    print("Setting up test environment...")

    yield  # This is where the test runs

    # Teardown code (e.g., remove directories, files, etc.)
    print("Tearing down test environment...")
