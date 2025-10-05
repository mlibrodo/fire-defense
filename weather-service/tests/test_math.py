from weather_service.math import (
    bearing_deg,
    compute_vpd_kpa,
    haversine_km,
    project_wind_along_cross,
    rollup,
)
from weather_service.typedefs import LatLon


def test_bearing_deg_basic():
    a = LatLon(0.0, 0.0)
    b = LatLon(1.0, 0.0)
    # From equator at 0E to 1N 0E should be bearing ~0 (north)
    assert abs(bearing_deg(a, b) - 0.0) < 1e-6


def test_haversine_km_symmetry():
    a = LatLon(37.7749, -122.4194)
    b = LatLon(34.0522, -118.2437)
    ab = haversine_km(a, b)
    ba = haversine_km(b, a)
    assert abs(ab - ba) < 1e-9
    assert ab > 500  # SF to LA ~559 km


def test_project_along_cross_conventions():
    # Wind from south (180 deg-from) blows toward north
    speed = 10.0
    dir_from = 180.0
    # Path bearing also due north: expect full tailwind ~ +10, zero cross
    along, cross = project_wind_along_cross(speed, dir_from, 0.0)
    assert abs(along - 10.0) < 1e-6
    assert abs(cross) < 1e-6


def test_compute_vpd_positive():
    vpd = compute_vpd_kpa(30.0, 10.0)
    assert vpd > 0.0


def test_rollup_aggregates(simple_series_point):
    r = rollup([simple_series_point])
    assert r.max_gust_ms == 7.0
    assert r.max_along_ms == 3.0
    assert r.max_cross_ms_abs == 4.0
    assert r.min_rh_pct == 20
