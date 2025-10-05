from datetime import datetime, timezone

from weather_service.models import (
    Rollups,
    SegmentMeta,
    SegmentRequest,
    SegmentResponse,
    SeriesPoint,
)
from weather_service.payloads import WX, Quality, Wind
from weather_service.service import WeatherService
from weather_service.typedefs import (
    LatLon,
    SamplingSpec,
    SamplingStrategy,
    TimeMode,
    TimeSpec,
    UnitsSpec,
)


class GoodAdapter:
    def get_segment_series(self, req: SegmentRequest) -> SegmentResponse:
        sp = SeriesPoint(
            time_utc=datetime(2025, 10, 4, 20, 0, tzinfo=timezone.utc),
            wind=Wind(5.0, 180.0, along_ms=5.0, cross_ms=0.0),
            wx=WX(),
            quality=Quality(source_token="good"),
        )
        return SegmentResponse(
            segment=SegmentMeta(bearing_deg=0.0, length_km=1.0),
            series=(sp,),
            rollups=Rollups(max_along_ms=5.0),
            meta_units=UnitsSpec(),
            horizon_hours=req.time.hours,
            sampling=req.sampling.strategy,
        )


class BadAdapter:
    def get_segment_series(self, req: SegmentRequest):
        raise RuntimeError("upstream failed")


def test_service_prefers_first_success():
    svc = WeatherService(adapters=[BadAdapter(), GoodAdapter()])
    req = SegmentRequest(
        a=LatLon(0, 0),
        b=LatLon(0, 1),
        time=TimeSpec(TimeMode.OBS, datetime.now(timezone.utc), 1),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )
    resp = svc.segment(req)
    assert resp.series[0].quality.source_token == "good"
