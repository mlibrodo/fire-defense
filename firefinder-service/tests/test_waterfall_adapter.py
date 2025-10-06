from typing import Iterable

from firefinder_service.adapters.waterfall import WaterfallAdapter, WaterfallPolicy
from firefinder_service.payloads import Incident
from firefinder_service.protocols import FireDataAdapter
from firefinder_service.service import InMemoryFireAdapter
from firefinder_service.typedefs import LatLon

# ---------- Tiny fake adapter utilities ----------


class FailingAdapter(FireDataAdapter):
    def search_incidents_within(
        self, center: LatLon, radius_miles: float
    ) -> Iterable[Incident]:
        raise RuntimeError("boom")

    def get_incident_by_id(self, incident_id: str):
        raise RuntimeError("boom")


class EmptyAdapter(FireDataAdapter):
    def search_incidents_within(
        self, center: LatLon, radius_miles: float
    ) -> Iterable[Incident]:
        return []

    def get_incident_by_id(self, incident_id: str):
        return None


def test_waterfall_falls_back_on_error(incident_point_near):
    good = InMemoryFireAdapter([incident_point_near])
    wf = WaterfallAdapter(
        adapters=[FailingAdapter(), good],
        policy=WaterfallPolicy(failover_on_error=True),
    )
    res = list(wf.search_incidents_within(LatLon(39.195, -120.235), 50))
    assert len(res) == 1 and res[0].id == "A"


def test_waterfall_falls_back_on_empty(incident_point_near):
    good = InMemoryFireAdapter([incident_point_near])
    wf = WaterfallAdapter(
        adapters=[EmptyAdapter(), good], policy=WaterfallPolicy(failover_on_empty=True)
    )
    res = list(wf.search_incidents_within(LatLon(39.195, -120.235), 50))
    assert len(res) == 1 and res[0].id == "A"


def test_waterfall_get_by_id_prefers_first_nonempty(incident_point_near):
    good = InMemoryFireAdapter([incident_point_near])
    wf = WaterfallAdapter(adapters=[EmptyAdapter(), good], policy=WaterfallPolicy())
    inc = wf.get_incident_by_id("A")
    assert inc is not None and inc.name == "Alpha"
