import pytest
from firefinder_service.models import NearbyFire, NearbyFiresRequest
from firefinder_service.typedefs import LatLon
from pydantic import ValidationError


def test_nearby_request_validation():
    req = NearbyFiresRequest(center=LatLon(39.1, -120.2), radius_miles=25)
    assert req.radius_miles == 25


def test_nearby_request_rejects_nonpositive_radius():
    with pytest.raises(ValidationError):
        NearbyFiresRequest(center=LatLon(39.1, -120.2), radius_miles=0)


def test_nearby_fire_serialization_roundtrip():
    nf = NearbyFire(
        id="X",
        name="X Fire",
        state="CA",
        county="Nevada",
        acres=123.4,
        containment_percent=55.0,
        distance_miles=1.23,
        sources=["rest"],
    )
    data = nf.model_dump()
    assert data["id"] == "X"
    assert data["acres"] == 123.4
