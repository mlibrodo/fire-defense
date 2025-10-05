from datetime import datetime, timezone

from weather_service.models import SegmentRequest
from weather_service.service import ServicePolicy, WeatherService
from weather_service.typedefs import (
    LatLon,
    SamplingSpec,
    SamplingStrategy,
    TimeMode,
    TimeSpec,
    UnitsSpec,
)


def test_policy_fallback_disabled(monkeypatch):
    # Force NDBCAdapter to always fail
    class FailingAdapter:
        def get_segment_series(self, req):
            raise RuntimeError("unavailable")

    svc = WeatherService(
        adapters=[FailingAdapter()],
        policy=ServicePolicy(allow_fallback_to_forecast=False),
    )
    req = SegmentRequest(
        a=LatLon(39.197, -120.238),
        b=LatLon(39.250, -120.150),
        time=TimeSpec(TimeMode.FORECAST, datetime.now(timezone.utc), 1),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )
    try:
        svc.segment(req)
    except RuntimeError as e:
        assert "unavailable" in str(e)
