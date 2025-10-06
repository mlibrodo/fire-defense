from firefinder_service.math import (
    haversine_miles,
    nearest_distance_to_polygon_vertices_miles,
)
from firefinder_service.typedefs import LatLon


def test_haversine_equator_lon_degree():
    a = LatLon(0, 0)
    b = LatLon(0, 1)
    d = haversine_miles(a, b)
    # ~69.17 mi per 1° lon at equator
    assert 68.5 <= d <= 69.8


def test_haversine_zero_distance():
    p = LatLon(37.0, -120.0)
    assert haversine_miles(p, p) == 0.0


def test_polygon_vertex_distance_basic(pt_center):
    poly = [
        [
            (pt_center.lat, pt_center.lon - 0.05),
            (pt_center.lat + 0.05, pt_center.lon - 0.05),
            (pt_center.lat + 0.05, pt_center.lon + 0.05),
            (pt_center.lat, pt_center.lon + 0.05),
        ]
    ]
    d = nearest_distance_to_polygon_vertices_miles(pt_center, poly)
    assert d is not None
    assert d < 10  # should be within a few miles
