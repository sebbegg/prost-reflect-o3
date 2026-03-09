from pathlib import Path

import pytest


@pytest.fixture()
def file_descriptor_path() -> str:
    p = Path(__file__).parent / "tests.bin"
    assert p.is_file()
    return str(p)
