"""Tests for firefinder-service."""


def test_main():
    """Test main function."""
    from . import main

    assert main.main is not None
