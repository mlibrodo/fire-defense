"""Tests for weather-service."""


def test_main():
    """Test main function."""
    from weather_service import main

    assert main.main is not None
