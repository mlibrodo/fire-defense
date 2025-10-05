# 🧪 Testing

We use `pytest` and `responses` to test math, adapters, and service routing in isolation.

## Install test deps

```bash
uv add --group dev pytest responses
uv run pytest -q
```

## Test matrix

| Layer      | Goal                                   | Tools             |
| ---------- | -------------------------------------- | ----------------- |
| `math`     | pure functions deterministic & correct | pytest            |
| `adapters` | parse & normalize upstream responses   | pytest, responses |
| `service`  | fallback routing across adapters       | pytest, responses |
| `models`   | dataclass integrity/serialization      | pytest            |

## Example: NDBC latest row parsing

```python
import responses
from weather_service.adapters import NDBCAdapter, NDBCStationResolver
from weather_service.models import SegmentRequest
from weather_service.typedefs import LatLon, SamplingSpec, SamplingStrategy, TimeMode, TimeSpec, UnitsSpec
from datetime import datetime, timezone

@responses.activate
def test_ndbc_adapter_latest_row_parsed_and_projected():
    station = "46026"
    url = f"https://www.ndbc.noaa.gov/data/realtime2/{station}.txt"
    sample = (
        "#YY  MM DD hh mm WDIR WSPD GST ...\n"
        "25 09 01 10 30 220  6.0  8.0\n"
    )
    responses.add(responses.GET, url, body=sample, status=200, content_type="text/plain")

    ad = NDBCAdapter(resolver=NDBCStationResolver(station_id=station), source_token="t")
    req = SegmentRequest(
        a=LatLon(39.0, -120.2), b=LatLon(39.1, -120.1),
        time=TimeSpec(TimeMode.OBS, datetime.now(timezone.utc), 1),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )
    resp = ad.get_segment_series(req)
    assert resp.series and resp.series[0].wind.speed_ms == 6.0
```

## Example: Service fallback to NWS

```python
import responses
from weather_service.service import WeatherService
from weather_service.adapters import NDBCAdapter, NDBCStationResolver, NWSAdapter
from weather_service.models import SegmentRequest
from weather_service.typedefs import LatLon, SamplingSpec, SamplingStrategy, TimeMode, TimeSpec, UnitsSpec
from datetime import datetime, timezone

@responses.activate
def test_service_fallback_when_ndbc_unavailable():
    # Minimal NWS mocks for fallback
    responses.add(responses.GET,
                  "https://api.weather.gov/points/39.197,-120.238",
                  json={"properties": {"forecastGridData": "https://api.weather.gov/gridpoints/REV/28,94"}},
                  status=200)
    responses.add(responses.GET,
                  "https://api.weather.gov/gridpoints/REV/28,94",
                  json={"properties": {"windSpeed": {"values": []}, "windDirection": {"values": []}}},
                  status=200)

    svc = WeatherService(adapters=[
        NDBCAdapter(resolver=NDBCStationResolver(station_id=None), source_token="obs"),
        NWSAdapter(source_token="fcst"),
    ])

    req = SegmentRequest(
        a=LatLon(39.197, -120.238), b=LatLon(39.250, -120.150),
        time=TimeSpec(TimeMode.FORECAST, datetime.now(timezone.utc), 1),
        sampling=SamplingSpec(SamplingStrategy.POINT_A, 10),
        units=UnitsSpec(),
    )
    resp = svc.segment(req)
    assert resp is not None
```
