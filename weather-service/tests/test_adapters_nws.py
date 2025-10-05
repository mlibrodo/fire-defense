from datetime import datetime, timezone

import responses
from weather_service.adapters.nws import NWSAdapter
from weather_service.models import SegmentRequest
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
def test_nws_adapter_parses_and_projects():
    # Mock points endpoint
    responses.add(
        responses.GET,
        POINTS_URL,
        json={"properties": {"forecastGridData": GRID_URL}},
        status=200,
        content_type="application/json",
    )

    # One hour of data: windSpeed km/h, windDirection deg
    def vt(h):
        return f"2025-10-04T{h:02d}:00:00+00:00/PT1H"

    grid_payload = {
        "properties": {
            "windSpeed": {"values": [{"validTime": vt(20), "value": 36.0}]},  # 10 m/s
            "windDirection": {"values": [{"validTime": vt(20), "value": 180.0}]},
            "temperature": {"values": [{"validTime": vt(20), "value": 25.0}]},
            "dewpoint": {"values": [{"validTime": vt(20), "value": 5.0}]},
            "relativeHumidity": {"values": [{"validTime": vt(20), "value": 20}]},
            "quantitativePrecipitation": {
                "values": [{"validTime": vt(20), "value": 0.0}]
            },
        }
    }

    responses.add(
        responses.GET,
        GRID_URL,
        json=grid_payload,
        status=200,
        content_type="application/json",
    )

    ad = NWSAdapter(source_token="opaque-fcst-test")

    req = SegmentRequest(
        a=LatLon(39.197, -120.238),
        b=LatLon(39.250, -120.150),
        time=TimeSpec(TimeMode.FORECAST, datetime.now(timezone.utc), 1),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )

    resp = ad.get_segment_series(req)
    assert len(resp.series) == 1
    p = resp.series[0]
    # 36 km/h => 10 m/s; wind from south along northward path => tailwind (~+10)
    assert abs(p.wind.speed_ms - 10.0) < 1e-6
    assert p.wind.along_ms is not None and p.wind.cross_ms is not None
    assert p.wx.rh_pct == 20
    assert p.wx.temp_c == 25.0
    assert p.wx.dewpoint_c == 5.0
    assert p.wx.precip_mm_1h == 0.0
    assert p.quality.source_token == "opaque-fcst-test"
