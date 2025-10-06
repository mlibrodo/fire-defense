from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, Iterable, List, Optional

try:
    from arcgis.features import FeatureLayer
    from arcgis.gis import GIS
except Exception:  # arcgis is optional
    GIS = None  # type: ignore
    FeatureLayer = None  # type: ignore

from ..payloads import Incident, IncidentGeometry
from ..protocols import FireDataAdapter  # protocol
from ..typedefs import LatLon


@dataclass
class ArcGISConfig:
    incidents_item_id: str  # '4181a117dc9e43db8598533e29972015'
    perimeters_item_id: Optional[str] = None  # '7c81ab78d8464e5c9771e49b64e834e9'
    portal_url: str = "https://www.arcgis.com"
    username: Optional[str] = None
    password: Optional[str] = None


class ArcGISFireAdapter(FireDataAdapter):
    """ArcGIS-backed adapter implementing FireDataAdapter."""

    def __init__(self, config: ArcGISConfig):
        if GIS is None:
            raise ImportError(
                "arcgis package not installed. Install with: pip install 'firefinder-service[arcgis]'"
            )
        self._cfg = config
        self._gis = GIS(self._cfg.portal_url, self._cfg.username, self._cfg.password)
        self._incidents_item = self._gis.content.get(self._cfg.incidents_item_id)
        if not self._incidents_item:
            raise ValueError(f"Incidents item not found: {self._cfg.incidents_item_id}")
        self._incidents_layer: FeatureLayer = self._incidents_item.layers[0]
        self._perims_layer: Optional[FeatureLayer] = None
        if self._cfg.perimeters_item_id:
            perim_item = self._gis.content.get(self._cfg.perimeters_item_id)
            if perim_item:
                self._perims_layer = perim_item.layers[0]

    def search_incidents_within(
        self, center: LatLon, radius_miles: float
    ) -> Iterable[Incident]:
        pt = {
            "x": float(center.lon),
            "y": float(center.lat),
            "spatialReference": {"wkid": 4326},
        }
        radius_m = float(radius_miles) * 1609.344

        out_fields = ",".join(
            [
                "IncidentName",
                "IRWINID",
                "POOState",
                "POOCounty",
                "CreatedOnDateTime",
                "PercentContained",
                "IncidentSize",
                "Latitude",
                "Longitude",
            ]
        )
        q_inc = self._incidents_layer.query(
            where="1=1",
            geometry=pt,
            geometry_type="esriGeometryPoint",
            spatial_rel="esriSpatialRelIntersects",
            distance=radius_m,
            units="esriSRUnit_Meter",
            out_fields=out_fields,
            return_geometry=False,
            order_by_fields="CreatedOnDateTime DESC",
            result_record_count=5000,
        )
        perim_map: Dict[str, float] = {}
        if self._perims_layer:
            q_per = self._perims_layer.query(
                where="1=1",
                geometry=pt,
                geometry_type="esriGeometryPoint",
                spatial_rel="esriSpatialRelIntersects",
                distance=radius_m,
                units="esriSRUnit_Meter",
                out_fields="IRWINID,IncidentName,GISAcres,DailyAcres,POOState,POOCounty",
                return_geometry=False,
            )
            for f in q_per.features:
                a = f.attributes
                key = a.get("IRWINID") or a.get("IncidentName")
                acres = a.get("GISAcres") or a.get("DailyAcres")
                if key and acres is not None:
                    perim_map.setdefault(str(key), float(acres))

        results: List[Incident] = []
        for f in q_inc.features:
            a = f.attributes
            inc_id = a.get("IRWINID") or str(a.get("OBJECTID"))
            name = a.get("IncidentName") or "Unknown"
            state = a.get("POOState")
            county = a.get("POOCounty")
            created = a.get("CreatedOnDateTime")
            containment = _to_float(a.get("PercentContained"))
            acres = perim_map.get(
                str(a.get("IRWINID") or a.get("IncidentName"))
            ) or _to_float(a.get("IncidentSize"))
            lat = _to_float(a.get("Latitude"))
            lon = _to_float(a.get("Longitude"))
            geom = (
                IncidentGeometry(point=LatLon(lat, lon))
                if (lat is not None and lon is not None)
                else None
            )
            results.append(
                Incident(
                    id=str(inc_id),
                    name=str(name),
                    state=state,
                    county=county,
                    created=created,
                    containment_percent=containment,
                    acres=acres,
                    geometry=geom,
                )
            )
        return results

    def get_incident_by_id(self, incident_id: str) -> Optional[Incident]:
        where = (
            f"IRWINID = '{incident_id}' OR OBJECTID = {safe_int_or_neg1(incident_id)}"
        )
        q = self._incidents_layer.query(
            where=where, out_fields="*", return_geometry=True
        )
        feats = q.features
        if not feats:
            return None
        a = feats[0].attributes
        inc_id = a.get("IRWINID") or str(a.get("OBJECTID"))
        name = a.get("IncidentName") or "Unknown"
        state = a.get("POOState")
        county = a.get("POOCounty")
        created = a.get("CreatedOnDateTime")
        containment = _to_float(a.get("PercentContained"))
        acres = _to_float(a.get("IncidentSize"))

        lat = _to_float(a.get("Latitude"))
        lon = _to_float(a.get("Longitude"))
        geom = (
            IncidentGeometry(point=LatLon(lat, lon))
            if (lat is not None and lon is not None)
            else None
        )

        return Incident(
            id=str(inc_id),
            name=str(name),
            state=state,
            county=county,
            created=created,
            containment_percent=containment,
            acres=acres,
            geometry=geom,
        )


def _to_float(v: Any) -> Optional[float]:
    try:
        if v is None:
            return None
        return float(v)
    except Exception:
        return None


def safe_int_or_neg1(s: str) -> int:
    try:
        return int(s)
    except Exception:
        return -1
