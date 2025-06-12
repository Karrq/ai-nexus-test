import pytest
import os

@pytest.fixture(scope="session", autouse=True)
def set_up_environment():
    # Ensure that the cargo project is built before running tests
    subprocess.run(["cargo", "build"], check=True)


