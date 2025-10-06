# 📐 Math & Geometry

- `haversine_miles(a, b)` — spherical great-circle distance (R=3958.7613 mi).
- `nearest_distance_to_polygon_vertices_miles(pt, polygon)` — fast vertex-only approximation.

Tradeoffs: vertex-only overestimates edge distance; future upgrade may use Shapely for edge-accurate distances.
