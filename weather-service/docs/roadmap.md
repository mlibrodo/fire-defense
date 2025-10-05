# 🗺️ Roadmap

- HRRR gridded model adapter (via NOMADS subset or AWS PDS GRIB2)
- MADIS quality-controlled obs adapter
- `schemas.py` (Pydantic v2) for public API serialization
- Fire risk indices: FFWI, ERC derived from existing fields
- Local caching & rate limiting layer
- Adapter capability filtering + `ServicePolicy` freshness rules
- Station resolvers for METAR/RAWS networks (adaptive selection)
- Structured logging & metrics (adapter latency, hit ratios, fallbacks)
