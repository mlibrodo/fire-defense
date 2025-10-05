from datetime import datetime, timezone

import responses
from weather_service.adapters import NDBCAdapter, NDBCStationResolver, NWSAdapter
from weather_service.models import SegmentRequest
from weather_service.service import WeatherService
from weather_service.typedefs import (
    LatLon,
    SamplingSpec,
    SamplingStrategy,
    TimeMode,
    TimeSpec,
    UnitsSpec,
)

POINTS_URL = "https://api.weather.gov/points/39.197,-120.238"
GRID_URL = "https://api.weather.gov/gridpoints/REV/28,94"


@responses.activate
def test_service_fallback_when_ndbc_unavailable():
    # Mock NWS minimal responses to allow fallback to succeed
    responses.add(
        responses.GET,
        POINTS_URL,
        json={"properties": {"forecastGridData": GRID_URL}},
        status=200,
    )
    responses.add(
        responses.GET,
        GRID_URL,
        json={
            "properties": {"windSpeed": {"values": []}, "windDirection": {"values": []}}
        },
        status=200,
    )

    svc = WeatherService(
        adapters=[
            NDBCAdapter(
                resolver=NDBCStationResolver(station_id=None),
                source_token="opaque-obs-1",
            ),
            NWSAdapter(source_token="opaque-fcst-1"),
        ]
    )

    req = SegmentRequest(
        a=LatLon(39.197, -120.238),
        b=LatLon(39.250, -120.150),
        time=TimeSpec(TimeMode.FORECAST, datetime.now(timezone.utc), 1),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )

    resp = svc.segment(req)
    assert resp is not None
