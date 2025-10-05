# Deploying Weather Service on Render (with uv)

This guide explains how to deploy the FastAPI + CLI Weather Service using [Render](https://render.com) and **uv** for dependency management.

---

## 🧰 Prerequisites

- The repository includes:
  - `weather_service/webapp.py` exporting `app`
  - `render.yaml` at the root
  - Optional `uv.lock` (recommended for reproducible builds)
- `pyproject.toml` defines:
  ```toml
  [project.optional-dependencies]
  web = ["fastapi>=0.110", "uvicorn>=0.30"]
  ```

---

## ⚙️ Build & start configuration

Render will automatically use these commands (defined in `render.yaml`):

**Build Command:**

```bash
pip install uv && uv sync --extra web --frozen
```

**Start Command:**

```bash
uv run uvicorn weather_service.webapp:app --host 0.0.0.0 --port $PORT
```

If you don’t yet have a `uv.lock`, create one locally:

```bash
uv lock
```

Commit it to your repo for reproducible builds.

---

## 🌐 Health Check

**Endpoint:**

```bash
GET /api/health
```

**Response:**

```json
{ "status": "ok" }
```

Set Render’s **Health Check Path** to `/api/health`.

---

## 🪴 Environment Variables

| Key                 | Default | Description                                |
| ------------------- | ------- | ------------------------------------------ |
| `NDBC_MAX_KM`       | 300     | Max search radius for nearest buoy         |
| `NDBC_STATIONS_CSV` | –       | Path to local buoy stations CSV (optional) |

---

## 🚀 Steps

1. Push repo to GitHub/GitLab.
2. In Render: **New + → Web Service → Connect Repo**.
3. Confirm Python 3.11+ runtime.
4. Use the build/start commands above.
5. Add environment variables if needed.
6. Deploy.

Render builds automatically, then launches your FastAPI app.

---

## 🔁 Redeploys & Monitoring

- **Auto-deploys:** enabled (`autoDeploy: true` in `render.yaml`)
- **Manual redeploy:** Render → Deploys → Manual Deploy
- **Logs:** View in Render dashboard
- **Metrics:** Optional performance and latency graphs

---

## 💰 Cost

The free tier is fine for testing, but may sleep.
Use a paid plan for production workloads.

---

© 2025 — Internal Fire Defense / Weather Service
