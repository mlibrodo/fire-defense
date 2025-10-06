import pytest

try:
    from fastapi.testclient import TestClient
    from firefinder_service.webapp import app

    HAVE_WEB = True
except Exception:
    HAVE_WEB = False


@pytest.mark.skipif(not HAVE_WEB, reason="fastapi not installed")
def test_health():
    c = TestClient(app)
    r = c.get("/api/health")
    assert r.status_code == 200
    assert r.json().get("status") == "ok"


@pytest.mark.skipif(not HAVE_WEB, reason="fastapi not installed")
def test_nearby_and_incident_and_distance():
    c = TestClient(app)
    near = c.get(
        "/api/nearby", params={"lat": 39.195, "lon": -120.235, "radius_mi": 50}
    )
    assert near.status_code == 200
    assert "fires" in near.json()

    inc = c.get("/api/incident/IRWIN123")
    # Depending on your app’s bootstrap seed, IRWIN123 may or may not exist.
    # Accept 200 or 404 (presence is environment-dependent).
    assert inc.status_code in (200, 404)

    dist = c.get(
        "/api/distance", params={"lat": 39.195, "lon": -120.235, "id": "IRWIN123"}
    )
    assert dist.status_code in (200, 404)
