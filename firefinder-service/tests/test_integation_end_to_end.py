import json
import re

import responses
from firefinder_service import NearbyFiresRequest
from firefinder_service.adapters.arcgis_rest import ArcGisRestAdapter, ArcGisRestConfig
from firefinder_service.adapters.waterfall import WaterfallAdapter, WaterfallPolicy
from firefinder_service.payloads import Incident, IncidentGeometry
from firefinder_service.service import FireFinderService, InMemoryFireAdapter
from firefinder_service.typedefs import LatLon

REST_URL = "https://example.com/FeatureServer/0/query"
URL_RE = re.compile(r"^https://example\.com/FeatureServer/0/query(?:\?.*)?$")


def _fake_feature(name="Garnet", state="NV", county="Humboldt", x=-118.125, y=41.678):
    return {
        "attributes": {
            "IncidentName": name,
            "POOState": state,
            "POOCounty": county,
            "CreatedOnDateTime": 1736146920000,
            "PercentContained": 25.0,
            "IncidentSize": 100.0,
            "IrwinID": "IRWIN-XYZ",
            "OBJECTID": 1,
        },
        "geometry": {"x": x, "y": y},
    }


@responses.activate
def test_end_to_end_waterfall_rest_then_memory():
    # REST returns an error (for every attempt) → waterfall must fall back to memory
    def always_error_cb(request):
        return (
            200,
            {},
            json.dumps(
                {
                    "error": {
                        "message": "Cannot perform query.",
                        "details": ["Invalid parameter"],
                    }
                }
            ),
        )

    responses.add_callback(
        responses.GET, URL_RE, callback=always_error_cb, content_type="application/json"
    )

    rest = ArcGisRestAdapter(
        ArcGisRestConfig(incidents_query_url=REST_URL, perimeters_query_url=None)
    )
    memory = InMemoryFireAdapter(
        [
            Incident(
                id="SEED",
                name="Seed Fire",
                geometry=IncidentGeometry(point=LatLon(39.2, -120.25)),
            )
        ]
    )

    svc = FireFinderService(
        adapter=WaterfallAdapter(
            adapters=[rest, memory],
            policy=WaterfallPolicy(failover_on_error=True, failover_on_empty=True),
        )
    )

    # ✅ Pass a NearbyFiresRequest model (not kwargs)
    resp = svc.search_nearby(
        NearbyFiresRequest(center=LatLon(39.195, -120.235), radius_miles=50)
    ).fires
    assert any(f.id == "SEED" for f in resp)
