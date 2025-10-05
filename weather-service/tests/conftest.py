from datetime import datetime, timezone

import pytest
from weather_service.models import SeriesPoint
from weather_service.payloads import WX, Quality, Wind
from weather_service.typedefs import LatLon, UnitsSpec


@pytest.fixture
def A():
    return LatLon(lat=39.197, lon=-120.238)


@pytest.fixture
def B():
    return LatLon(lat=39.250, lon=-120.150)


@pytest.fixture
def units():
    return UnitsSpec()


@pytest.fixture
def simple_series_point():
    t = datetime(2025, 10, 4, 20, 0, tzinfo=timezone.utc)
    wind = Wind(
        speed_ms=5.0, dir_from_deg=180.0, gust_ms=7.0, along_ms=3.0, cross_ms=4.0
    )
    wx = WX(rh_pct=20, temp_c=25.0, dewpoint_c=5.0, vpd_kpa=2.0, precip_mm_1h=0.0)
    q = Quality(data_age_min=5, source_token="opaque-test", qflags=("ok",))
    return SeriesPoint(time_utc=t, wind=wind, wx=wx, quality=q)
