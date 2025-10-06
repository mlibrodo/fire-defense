from __future__ import annotations

import argparse
import json
import sys
from typing import Optional

from .models import NearbyFiresRequest
from .service import build_service
from .typedefs import LatLon


def run_cli_nearby(lat: float, lon: float, radius_mi: float) -> None:
    svc = build_service()
    req = NearbyFiresRequest(center=LatLon(lat, lon), radius_miles=radius_mi)
    resp = svc.search_nearby(req)
    print(json.dumps(resp.model_dump(), indent=2))


def run_cli_distance(lat: float, lon: float, incident_id: str) -> None:
    svc = build_service()
    out = svc.distance_to_incident(LatLon(lat, lon), incident_id)
    if not out:
        print("Incident not found or no geometry", file=sys.stderr)
        sys.exit(1)
    print(json.dumps(out.model_dump(), indent=2))


def run_web(host: str = "0.0.0.0", port: int = 8100) -> int:
    import uvicorn

    from .webapp import app

    uvicorn.run(app, host=host, port=port)
    return 0


def main(argv: Optional[list[str]] = None) -> None:
    parser = argparse.ArgumentParser(
        prog="firefinder", description="FireFinder CLI / Web"
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_cli = sub.add_parser("cli", help="Run a one-off FireFinder query")
    p_cli.add_argument("--mode", choices=["nearby", "distance"], default="nearby")
    p_cli.add_argument("--lat", type=float, required=True)
    p_cli.add_argument("--lon", type=float, required=True)
    p_cli.add_argument("--radius-mi", type=float, default=25.0)
    p_cli.add_argument("--id", type=str, default=None)

    p_web = sub.add_parser("web", help="Run FastAPI web app")
    p_web.add_argument("--host", default="0.0.0.0")
    p_web.add_argument("--port", type=int, default=8100)

    args = parser.parse_args(argv)

    if args.cmd == "cli":
        if args.mode == "nearby":
            return run_cli_nearby(args.lat, args.lon, args.radius_mi)
        else:
            if not args.id:
                parser.error("--id is required when mode=distance")
            return run_cli_distance(args.lat, args.lon, args.id)
    elif args.cmd == "web":
        return run_web(args.host, args.port)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
