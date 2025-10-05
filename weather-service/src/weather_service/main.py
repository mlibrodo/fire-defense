from __future__ import annotations

import argparse
from dataclasses import asdict
from datetime import datetime, timezone
from typing import Optional

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


def build_service() -> WeatherService:
    resolver = NDBCStationResolver(station_id=None)
    return WeatherService(
        adapters=[
            NDBCAdapter(resolver=resolver, source_token="opaque-obs-1"),
            NWSAdapter(source_token="opaque-fcst-1"),
        ]
    )


def run_cli(
    a_lat: float,
    a_lon: float,
    b_lat: float,
    b_lon: float,
    hours: int = 24,
    mode: str = "forecast",
    level_m_agl: int = 10,
) -> None:
    svc = build_service()
    req = SegmentRequest(
        a=LatLon(a_lat, a_lon),
        b=LatLon(b_lat, b_lon),
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
    import json

    print(json.dumps(asdict(resp), indent=2, default=str))


def run_web(host: str = "0.0.0.0", port: int = 8100) -> None:
    import uvicorn
    from weather_service.webapp import app

    uvicorn.run(app, host=host, port=port)


def main(argv: Optional[list[str]] = None) -> None:
    parser = argparse.ArgumentParser(
        prog="weather_service", description="Weather Service CLI / Web"
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_cli = sub.add_parser("cli", help="Run a one-off wind query")
    p_cli.add_argument("--alat", type=float, required=True)
    p_cli.add_argument("--alon", type=float, required=True)
    p_cli.add_argument("--blat", type=float, required=True)
    p_cli.add_argument("--blon", type=float, required=True)
    p_cli.add_argument("--hours", type=int, default=24)
    p_cli.add_argument("--mode", choices=["forecast", "obs"], default="forecast")
    p_cli.add_argument("--level_m_agl", type=int, default=10)

    p_web = sub.add_parser("web", help="Run FastAPI web app")
    p_web.add_argument("--host", default="0.0.0.0")
    p_web.add_argument("--port", type=int, default=8100)

    args = parser.parse_args(argv)
    if args.cmd == "cli":
        run_cli(
            args.alat,
            args.alon,
            args.blat,
            args.blon,
            args.hours,
            args.mode,
            args.level_m_agl,
        )
    elif args.cmd == "web":
        run_web(args.host, args.port)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
