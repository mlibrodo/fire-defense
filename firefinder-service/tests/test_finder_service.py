from firefinder_service.models import NearbyFiresRequest


def test_service_search_nearby_sort_and_fields(demo_service, pt_center):
    resp = demo_service.search_nearby(
        NearbyFiresRequest(center=pt_center, radius_miles=50)
    )
    # Basic shape
    assert hasattr(resp, "fires")
    assert len(resp.fires) >= 2
    # Sorted by distance
    distances = [f.distance_miles for f in resp.fires if f.distance_miles is not None]
    assert distances == sorted(distances)
    # Severity derived
    assert all(f.severity in {"low", "medium", "high", "unknown"} for f in resp.fires)


def test_service_distance_to_incident_chooses_min_basis(demo_service, pt_center):
    out = demo_service.distance_to_incident(pt_center, "A")
    assert out is not None
    assert out.basis in {"point", "polygon_vertices"}
    assert out.distance_miles >= 0
