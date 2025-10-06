import pytest
from firefinder_service.payloads import Incident, IncidentGeometry
from firefinder_service.service import FireFinderService, InMemoryFireAdapter
from firefinder_service.typedefs import LatLon

# ---------- Shared test data ----------


@pytest.fixture
def pt_center() -> LatLon:
    # Near Truckee, CA
    return LatLon(39.195, -120.235)


@pytest.fixture
def incident_point_near() -> Incident:
    return Incident(
        id="A",
        name="Alpha",
        acres=1500,
        containment_percent=40.0,
        geometry=IncidentGeometry(point=LatLon(39.200, -120.250)),
    )


@pytest.fixture
def incident_poly_near() -> Incident:
    return Incident(
        id="B",
        name="Bravo",
        acres=50,
        containment_percent=10.0,
        geometry=IncidentGeometry(
            perimeters=[
                [
                    (39.250, -120.300),
                    (39.260, -120.280),
                    (39.240, -120.270),
                ]
            ]
        ),
    )


@pytest.fixture
def incident_nogeom() -> Incident:
    return Incident(
        id="C",
        name="NoGeom",
        acres=None,
        containment_percent=None,
        geometry=None,
    )


# ---------- Adapters & Services ----------


@pytest.fixture
def inmemory_adapter(incident_point_near, incident_poly_near, incident_nogeom):
    return InMemoryFireAdapter(
        incidents=[incident_point_near, incident_poly_near, incident_nogeom]
    )


@pytest.fixture
def demo_service(inmemory_adapter) -> FireFinderService:
    return FireFinderService(adapter=inmemory_adapter)
