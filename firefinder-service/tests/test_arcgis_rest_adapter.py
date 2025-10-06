import json
import re

import responses
from firefinder_service.adapters.arcgis_rest import ArcGisRestAdapter, ArcGisRestConfig
from firefinder_service.typedefs import LatLon

REST_URL = "https://example.com/FeatureServer/0/query"
URL_RE = re.compile(r"^https://example\.com/FeatureServer/0/query(?:\?.*)?$")


def _fake_feature(name="Garnet"):
    return {
        "attributes": {
            "IncidentName": name,
            "POOState": "NV",
            "POOCounty": "Humboldt",
            "CreatedOnDateTime": 1736146920000,
            "PercentContained": 25.0,
            "IncidentSize": 100.0,
            "IrwinID": "IRWIN-XYZ",
            "OBJECTID": 99,
        },
        "geometry": {"x": -118.125, "y": 41.678},
    }


@responses.activate
def test_rest_search_basic_success():
    cfg = ArcGisRestConfig(incidents_query_url=REST_URL, perimeters_query_url=None)
    adapter = ArcGisRestAdapter(cfg)

    # Register enough responses to satisfy fallback variants (up to 3)
    for _ in range(3):
        responses.add(
            responses.GET,
            URL_RE,
            json={"features": [_fake_feature()]},
            status=200,
        )

    feats = list(adapter.search_incidents_within(LatLon(39.195, -120.235), 50))
    assert len(feats) == 1
    assert feats[0].name == "Garnet"


@responses.activate
def test_rest_get_incident_by_id_where_clause():
    cfg = ArcGisRestConfig(incidents_query_url=REST_URL, perimeters_query_url=None)
    adapter = ArcGisRestAdapter(cfg)

    def _cb(req):
        # Make sure our query includes IrwinID or name LIKE
        assert "where=" in req.url
        body = {"features": [_fake_feature(name="Alpha")]}
        return (200, {}, json.dumps(body))

    responses.add_callback(
        responses.GET, REST_URL, callback=_cb, content_type="application/json"
    )

    inc = adapter.get_incident_by_id("IRWIN-XYZ")
    assert inc is not None
    assert inc.name == "Alpha"


@responses.activate
def test_rest_handles_error_body():
    cfg = ArcGisRestConfig(incidents_query_url=REST_URL, perimeters_query_url=None)
    adapter = ArcGisRestAdapter(cfg)

    responses.add(
        responses.GET,
        REST_URL,
        json={
            "error": {
                "message": "Cannot perform query.",
                "details": ["Invalid parameter"],
            }
        },
        status=200,
    )

    try:
        list(adapter.search_incidents_within(LatLon(39.195, -120.235), 50))
        assert False, "expected RuntimeError"
    except RuntimeError as e:
        assert "Cannot perform query" in str(e)
