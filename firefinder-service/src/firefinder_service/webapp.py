from __future__ import annotations

from pathlib import Path

from fastapi import APIRouter, FastAPI, HTTPException
from fastapi.responses import HTMLResponse, JSONResponse

from .models import NearbyFiresRequest
from .service import build_service
from .typedefs import LatLon

# Routers
api = APIRouter(prefix="/api", tags=["api"])
pages = APIRouter(tags=["pages"])
health = APIRouter()

svc = build_service()


@health.get("/api/health", tags=["system"])
def health_check():
    return {"status": "ok"}


@api.get("/nearby")
def api_nearby(lat: float, lon: float, radius_mi: float):
    try:
        req = NearbyFiresRequest(center=LatLon(lat, lon), radius_miles=radius_mi)
        resp = svc.search_nearby(req)
        return JSONResponse(resp.model_dump())
    except Exception as e:
        raise HTTPException(status_code=502, detail=str(e))


@api.get("/distance")
def api_distance(lat: float, lon: float, id: str):
    try:
        out = svc.distance_to_incident(LatLon(lat, lon), id)
        if not out:
            return JSONResponse({"detail": "not found or no geometry"}, status_code=404)
        return JSONResponse(out.model_dump())
    except Exception as e:
        raise HTTPException(status_code=502, detail=str(e))


@api.get("/incident/{incident_id}")
def api_incident(incident_id: str):
    """Return a single incident by ID (e.g., IRWINID)."""
    inc = svc.get_incident_by_id(incident_id)
    if not inc:
        raise HTTPException(status_code=404, detail="not found")

    # Minimal serialization (expand as needed)
    out = {
        "id": inc.id,
        "name": inc.name,
        "state": inc.state,
        "county": inc.county,
        "created": inc.created,
        "containment_percent": inc.containment_percent,
        "acres": inc.acres,
        # geometry is optional; include point if present for convenience
        "point": {
            "lat": getattr(inc.geometry.point, "lat", None)
            if inc.geometry and inc.geometry.point
            else None,
            "lon": getattr(inc.geometry.point, "lon", None)
            if inc.geometry and inc.geometry.point
            else None,
        },
    }
    return JSONResponse(out)


@pages.get("/", response_class=HTMLResponse)
def index():
    html_path = Path(__file__).parent.parent / "templates" / "index.html"
    if html_path.exists():
        return HTMLResponse(html_path.read_text(encoding="utf-8"))
    return HTMLResponse("<h1>FireFinder</h1><p>Template not found.</p>")


def create_app() -> FastAPI:
    app = FastAPI(title="FireFinder Service", version="0.2.0")
    app.include_router(api)
    app.include_router(pages)
    app.include_router(health)
    return app


app = create_app()


def run_web(host: str = "0.0.0.0", port: int = 8100) -> int:
    import uvicorn

    uvicorn.run("firefinder_service.webapp:app", host=host, port=port, reload=False)
    return 0
