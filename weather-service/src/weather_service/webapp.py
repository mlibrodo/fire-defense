from __future__ import annotations

import os
from datetime import datetime, timezone
from pathlib import Path

from fastapi import APIRouter, FastAPI, HTTPException
from fastapi.encoders import jsonable_encoder
from fastapi.responses import HTMLResponse, JSONResponse
from weather_service import (
    LatLon,
    SamplingSpec,
    SamplingStrategy,
    SegmentRequest,
    TimeMode,
    TimeSpec,
    UnitsSpec,
    WeatherService,
)
from weather_service.adapters import NDBCAdapter, NDBCStationResolver, NWSAdapter


# ---------------------------------------------------------------------------
# Service bootstrap (shared for all routes)
# ---------------------------------------------------------------------------
def build_service() -> WeatherService:
    """Create and return a WeatherService instance with both NDBC and NWS adapters."""
    stations_csv = os.getenv("NDBC_STATIONS_CSV")
    resolver = NDBCStationResolver(
        station_id=None,
        csv_path=stations_csv,
        max_km=float(os.getenv("NDBC_MAX_KM", "300")),
    )
    return WeatherService(
        adapters=[
            NDBCAdapter(resolver=resolver, source_token="opaque-obs-1"),
            NWSAdapter(source_token="opaque-fcst-1"),
        ]
    )


# ---------------------------------------------------------------------------
# Routers
# ---------------------------------------------------------------------------
api = APIRouter(prefix="/api", tags=["api"])
pages = APIRouter(tags=["pages"])

svc = build_service()


@api.get("/wind")
def api_wind(
    alat: float,
    alon: float,
    blat: float,
    blon: float,
    hours: int = 24,
    mode: str = "forecast",
    level_m_agl: int = 10,
):
    """REST endpoint that returns wind data between two coordinates."""
    try:
        req = SegmentRequest(
            a=LatLon(alat, alon),
            b=LatLon(blat, blon),
            time=TimeSpec(
                TimeMode.FORECAST if mode.lower().startswith("f") else TimeMode.OBS,
                start=datetime.now(timezone.utc),
                hours=hours,
            ),
            sampling=SamplingSpec(
                strategy=SamplingStrategy.POINT_A, level_m_agl=level_m_agl
            ),
            units=UnitsSpec(),
        )
        resp = svc.segment(req)
        data = jsonable_encoder(resp)
        return JSONResponse(content=data)
    except Exception as e:
        raise HTTPException(status_code=502, detail=str(e))


@pages.get("/", response_class=HTMLResponse)
def index():
    """Serve the static HTML page with the wind query form."""
    html_path = Path(__file__).parent.parent / "templates" / "index.html"
    if html_path.exists():
        return HTMLResponse(html_path.read_text(encoding="utf-8"))
    return HTMLResponse("<h1>Weather Service</h1><p>Template not found.</p>")


# ---------------------------------------------------------------------------
# App Factory
# ---------------------------------------------------------------------------
def create_app() -> FastAPI:
    """Return a FastAPI app combining page + API routers."""
    app = FastAPI(title="Weather Service", version="0.1.0")
    app.include_router(api)
    app.include_router(pages)
    return app


# Expose app for uvicorn
app = create_app()
