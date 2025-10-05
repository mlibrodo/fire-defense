from dataclasses import is_dataclass
from datetime import datetime, timezone

from weather_service.models import SegmentRequest
from weather_service.typedefs import (
    LatLon,
    SamplingSpec,
    SamplingStrategy,
    TimeMode,
    TimeSpec,
    UnitsSpec,
)


def test_typedefs_are_dataclasses():
    assert is_dataclass(LatLon(0, 0))
    assert is_dataclass(TimeSpec(TimeMode.FORECAST, datetime.now(timezone.utc), 6))
    assert is_dataclass(SamplingSpec(SamplingStrategy.POINT_A, 10))
    assert is_dataclass(UnitsSpec())


def test_models_request_wiring():
    a = LatLon(39.0, -120.0)
    b = LatLon(39.1, -119.9)
    req = SegmentRequest(
        a=a,
        b=b,
        time=TimeSpec(TimeMode.FORECAST, datetime.now(timezone.utc), 3),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )
    assert req.a == a and req.b == b
