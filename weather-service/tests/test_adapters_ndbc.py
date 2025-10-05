from datetime import datetime, timezone

import responses
from weather_service.adapters import NDBCAdapter, NDBCStationResolver
from weather_service.models import SegmentRequest
from weather_service.typedefs import (
    LatLon,
    SamplingSpec,
    SamplingStrategy,
    TimeMode,
    TimeSpec,
    UnitsSpec,
)

NDBC_URL = "https://www.ndbc.noaa.gov/data/realtime2/1.txt"

SAMPLE = "#YY  MM DD hh mm WDIR WSPD GST\n2025 10 04 20 00 180 5.0 7.0\n"


@responses.activate
def test_ndbc_adapter_latest_row_parsed_and_projected():
    responses.add(
        responses.GET, NDBC_URL, body=SAMPLE, status=200, content_type="text/plain"
    )
    ad = NDBCAdapter(
        resolver=NDBCStationResolver(station_id="1"), source_token="opaque-obs-test"
    )
    req = SegmentRequest(
        a=LatLon(39.197, -120.238),
        b=LatLon(39.250, -120.150),
        time=TimeSpec(TimeMode.OBS, datetime.now(timezone.utc), 1),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )
    resp = ad.get_segment_series(req)
    assert len(resp.series) == 1
    p = resp.series[0]
    assert p.wind.speed_ms == 5.0
    assert p.wind.gust_ms == 7.0
    assert p.wind.along_ms is not None and p.wind.cross_ms is not None
    assert p.quality.source_token == "opaque-obs-test"
