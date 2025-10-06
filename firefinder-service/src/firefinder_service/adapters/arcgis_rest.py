from __future__ import annotations

import json
import re
from dataclasses import dataclass
from typing import Iterable, Optional

import requests

from ..payloads import Incident, IncidentGeometry
from ..protocols import FireDataAdapter
from ..typedefs import LatLon

# ---- Defaults: NIFC/WFIGS public layers (no auth needed) --------------------

# Current Wildland Fire Incident Locations (points)
DEFAULT_INCIDENTS_URL = (
    "https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/"
    "WFIGS_Incident_Locations_Current/FeatureServer/0/query"
)

# WFIGS Interagency Fire Perimeters (current year to date) (polygons)
# Optional for enrichment; you can leave it None to skip perimeter hits.
DEFAULT_PERIMETERS_URL = (
    "https://services3.arcgis.com/T4QMspbfLg3qTGWY/ArcGIS/rest/services/"
    "WFIGS_Interagency_Fire_Perimeters_to_Date_2025/FeatureServer/0/query"
)


@dataclass(frozen=True)
class ArcGisRestConfig:
    incidents_query_url: str = DEFAULT_INCIDENTS_URL
    perimeters_query_url: Optional[str] = DEFAULT_PERIMETERS_URL  # can be None
    timeout_s: float = 20.0
    # If your org requires a token, add a field here and include &token=... in requests.


class ArcGisRestAdapter(FireDataAdapter):
    """
    Lightweight adapter that talks directly to ArcGIS FeatureServer /query endpoints.
    It returns normalized Incident objects consumed by FireFinderService.
    """

    def __init__(self, cfg: ArcGisRestConfig | None = None):
        self.cfg = cfg or ArcGisRestConfig()

    # ----------------------------- Public API ---------------------------------

    def search_incidents_within(
        self, center: LatLon, radius_miles: float
    ) -> Iterable[Incident]:
        base = {
            "where": "1=1",
            "outFields": "*",
            "returnGeometry": "true",
            "geometry": json.dumps(
                {"x": center.lon, "y": center.lat, "spatialReference": {"wkid": 4326}}
            ),
            "geometryType": "esriGeometryPoint",
            "inSR": 4326,
            "spatialRel": "esriSpatialRelIntersects",
            "f": "json",
        }

        variants = [
            # A) miles + geodesic (preferred)
            {
                **base,
                "distance": radius_miles,
                "units": "esriSRUnit_StatuteMile",
                "geodesic": "true",
            },
            # B) meters
            {**base, "distance": radius_miles * 1609.344, "units": "esriSRUnit_Meter"},
            # C) no units (uses layer units)
            {**base, "distance": radius_miles},
        ]

        last_err = None
        for p in variants:
            try:
                data = self._get(self.cfg.incidents_query_url, p)
                feats = data.get("features", [])
                if feats is None:
                    feats = []
                # Success — map features and return
                return [self._feature_to_incident(ft) for ft in feats]
            except Exception as e:
                last_err = e
                continue

        # If all variants failed, raise the last error for visibility
        raise last_err or RuntimeError("ArcGIS query failed (all parameter variants)")

    def get_incident_by_id(self, incident_id: str) -> Optional[Incident]:
        """
        Look up by IrwinID/IRWINID/GlobalID/OBJECTID; fall back to name match if needed.
        """
        candidates = [
            f"IrwinID='{incident_id}'",
            f"IRWINID='{incident_id}'",
            f"GlobalID='{incident_id}'",
        ]

        # OBJECTID numeric?
        if incident_id.isdigit():
            candidates.append(f"OBJECTID={incident_id}")

        # If not a GUID-ish value, also try name contains (escape single quotes)
        if not _looks_like_guid(incident_id):
            like = re.sub(r"'", "''", incident_id)
            candidates.append(f"UPPER(IncidentName) LIKE UPPER('%{like}%')")

        where = " OR ".join(candidates)
        params = {
            "where": where,
            "outFields": "*",
            "returnGeometry": "true",
            "f": "json",
        }
        data = self._get(self.cfg.incidents_query_url, params)
        feats = data.get("features", [])
        if not feats:
            return None
        return self._feature_to_incident(feats[0])

    # ----------------------------- Helpers ------------------------------------

    def _get(self, url: str, params: dict) -> dict:
        r = requests.get(url, params=params, timeout=self.cfg.timeout_s)
        r.raise_for_status()
        data = r.json()
        if "error" in data:
            # ArcGIS errors come back 200 with "error": {...}
            msg = data["error"].get("message", "ArcGIS REST error")
            details = "; ".join(data["error"].get("details", []))
            raise RuntimeError(f"{msg}: {details}")
        return data

    def _feature_to_incident(self, ft: dict) -> Incident:
        a = ft.get("attributes", {}) or {}
        g = ft.get("geometry", {}) or {}

        name = a.get("IncidentName")
        state = a.get("POOState")
        county = a.get("POOCounty")
        created = _epoch_ms_to_iso(a.get("CreatedOnDateTime"))
        containment = _first_number(a.get("PercentContained"))
        # Prefer GISAcres (perimeter-derived) over IncidentSize if present
        acres = _first_number(a.get("GISAcres")) or _first_number(a.get("IncidentSize"))

        # Pick an ID (IrwinID > GlobalID > OBJECTID)
        inc_id = (
            a.get("IrwinID")
            or a.get("IRWINID")
            or a.get("GlobalID")
            or str(a.get("OBJECTID"))
        )

        # Point geometry from feature (x=lon, y=lat)
        point = None
        if "x" in g and "y" in g:
            point = LatLon(lat=float(g["y"]), lon=float(g["x"]))

        geom = IncidentGeometry(point=point) if point else None
        return Incident(
            id=inc_id,
            name=name or "(unknown)",
            state=state,
            county=county,
            created=created,
            containment_percent=containment,
            acres=acres,
            geometry=geom,
        )


# ------------------------------ Utils ----------------------------------------


def _epoch_ms_to_iso(ms: Optional[int | float | str]) -> Optional[str]:
    if ms is None:
        return None
    try:
        ms = int(ms)
    except Exception:
        return None
    # ArcGIS stores UTC epoch millis
    import datetime as _dt

    return (
        _dt.datetime.utcfromtimestamp(ms / 1000)
        .replace(tzinfo=_dt.timezone.utc)
        .isoformat()
    )


def _first_number(val) -> Optional[float]:
    try:
        if val is None:
            return None
        return float(val)
    except Exception:
        return None


_GUID_RE = re.compile(
    r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$"
)


def _looks_like_guid(s: str) -> bool:
    return bool(_GUID_RE.match(s or ""))
