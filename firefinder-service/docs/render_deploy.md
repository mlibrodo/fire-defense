# Deploying FireFinder Service on Render (with uv)

This guide explains how to deploy the FastAPI + CLI **FireFinder Service** using [Render](https://render.com) and **uv** for dependency management.

---

## 🧰 Prerequisites

Ensure your repository includes:

- `firefinder_service/webapp.py` exporting a FastAPI `app`
- `render.yaml` at the root (controls Render deployment)
- Optional `uv.lock` for reproducible builds
- `pyproject.toml` defining `web` extras, for example:

```toml
[project.optional-dependencies]
web = ["fastapi>=0.118.0", "uvicorn[standard]>=0.37.0"]
```

> 💡 The `uv` tool manages virtual environments and dependency resolution; Render runs these commands automatically.

---

## ⚙️ Build & Start Configuration

Render automatically uses your **build** and **start** commands defined in `render.yaml`.

**Build Command:**

```bash
pip install uv && uv sync --extra web --frozen
```

**Start Command:**

```bash
uv run uvicorn firefinder_service.webapp:app --host 0.0.0.0 --port $PORT
```

If you don’t yet have a `uv.lock`, create one locally before committing:

```bash
uv lock
```

This ensures **deterministic dependency resolution** on Render’s build machines.

---

## 🌐 Health Check

**Endpoint:**

```bash
GET /api/health
```

**Expected Response:**

```json
{ "status": "ok" }
```

Set Render’s **Health Check Path** to `/api/health` for continuous availability verification.

---

## 🪴 Environment Variables

FireFinder currently doesn’t require mandatory environment variables, but these placeholders will be added as the system evolves:

| Key                         | Default    | Description                                                  |
| --------------------------- | ---------- | ------------------------------------------------------------ |
| `FIREFINDER_ADAPTER_MODE`   | `rest`     | Selects which adapter to load (`rest`, `sdk`, or `inmemory`) |
| `FIREFINDER_DEBUG`          | `false`    | Enables verbose logging                                      |
| `FIREFINDER_PORT`           | `8100`     | Custom port for FastAPI app                                  |
| `FIREFINDER_PERIMETERS_URL` | _optional_ | Override for WFIGS perimeter layer                           |
| `FIREFINDER_INCIDENTS_URL`  | _optional_ | Override for WFIGS incident layer                            |

> If no variables are defined, defaults from `ArcGisRestConfig` are used.

---

## 🚀 Deployment Steps

1. **Push your repo** to GitHub or GitLab.
2. Log in to [Render](https://render.com).
3. Select **New + → Web Service → Connect Repo**.
4. Confirm the **Python 3.11+ runtime**.
5. Use the **Build** and **Start** commands listed above.
6. Optionally add environment variables for configuration overrides.
7. Click **Deploy**.

Render will automatically build the environment, install dependencies using `uv`, and launch your FastAPI app.

---

## 🔁 Redeploys & Monitoring

- **Auto-deploys:** Enable in your Render dashboard (`autoDeploy: true` in `render.yaml`)
- **Manual redeploy:** Render → Deploys → Manual Deploy
- **Logs:** Stream live build and runtime logs in Render UI
- **Metrics:** Render offers latency, throughput, and uptime monitoring

For debugging, logs are also accessible via `uvicorn` console output.

---

## 🧩 Example render.yaml

Example minimal configuration file:

```yaml
services:
  - type: web
    name: firefinder-service
    env: python
    buildCommand: pip install uv && uv sync --extra web --frozen
    startCommand: uv run uvicorn firefinder_service.webapp:app --host 0.0.0.0 --port $PORT
    plan: free
    autoDeploy: true
    healthCheckPath: /api/health
    envVars:
      - key: FIREFINDER_ADAPTER_MODE
        value: rest
```

Commit this `render.yaml` to your repo root.

---

## 🧰 Debugging common issues

| Symptom                                  | Fix                                                                                    |
| ---------------------------------------- | -------------------------------------------------------------------------------------- |
| App fails to import `firefinder_service` | Ensure `src/` is in your package path (`[tool.setuptools] package-dir = {"" = "src"}`) |
| Missing `fastapi` or `uvicorn`           | Install web extras: `uv sync --extra web`                                              |
| ArcGIS REST request timeout              | Verify public dataset availability or increase timeout in `ArcGisRestConfig`           |
| `Invalid URL` errors                     | Ensure `/ArcGIS/rest/` casing in URLs                                                  |

---

## 💰 Cost & Scaling

Render’s **free tier** is suitable for development or low-volume usage but may sleep during inactivity.
For production deployments, upgrade to a paid plan to keep the service always on.

---

## 🔎 References

- Render Docs: https://render.com/docs
- NIFC Open Data Portal: https://data-nifc.opendata.arcgis.com
- WFIGS Incident Dataset: https://www.arcgis.com/home/item.html?id=4181a117dc9e43db8598533e29972015
- WFIGS Perimeter Dataset: https://www.arcgis.com/home/item.html?id=7c81ab78d8464e5c9771e49b64e834e9

---

© 2025 — FireFinder Service / Internal Fire Defense Project
