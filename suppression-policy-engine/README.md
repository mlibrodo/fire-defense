# 🔥 Suppression Policy Engine

The **Suppression Policy Engine (SPE)** orchestrates automated wildfire defense actions across connected
devices and installations. It translates **high-level fire suppression policies** (Observe → Suppress)
into **device-level commands** executed through ControlByWeb relays.

---

## 🎯 Purpose

This service provides a unified interface for **policy evaluation and execution**, handling:

- **Installation → Device → Account resolution**
- **Dynamic Device Access Token (DAT)** creation and cleanup
- **Relay control** via ControlByWeb Cloud API
- **Policy orchestration** through an `Engine` and `Runner` abstraction

The result: a **policy-driven control layer** for distributed wildfire defense systems.

---

## 🧩 Architecture Overview

```
installation_id ─┬─> resolver (account.rs) ─┬─> account_id
                 │                          └─> device_id
                 │
                 ├─> policy (policy.rs) ─────> level + actions
                 │
                 ├─> engine (engine.rs) ─────> evaluates → plan
                 │
                 ├─> runner (suppression_policy_runner.rs)
                 │        applies engine plan via DAL
                 │
                 └─> device_abstraction_layer/
                          ├─ drivers/
                          │   └─ control_by_web/
                          │        ├─ client.rs        → executes device commands
                          │        ├─ token.rs         → manages OAuth tokens
                          │        ├─ device_access.rs → creates DATs (Device Access Tokens)
                          │        ├─ relay_plan.rs    → determines which relays toggle
                          │        └─ account.rs       → installation ↔ account/device mapping
                          └─ trait DeviceDriver        → common interface
```

---

## ⚙️ Environment Variables

| Variable                 | Description                                  | Example                              |
| ------------------------ | -------------------------------------------- | ------------------------------------ |
| `CBW_BASE_URL`           | ControlByWeb Cloud API base URL              | `https://api.controlbyweb.cloud/api` |
| `CBW_ACCOUNT_ID`         | Primary ControlByWeb account ID              | `3023095475`                         |
| `CBW_ACCOUNT_ID_DEFAULT` | Fallback account ID (optional)               | `3023095475`                         |
| `CBW_DEVICE_ID_DEFAULT`  | Default device ID for fallback installations | `2168150121`                         |
| `CBW_USERNAME`           | ControlByWeb username                        | `Scayolle`                           |
| `CBW_PASSWORD`           | ControlByWeb password                        | `•••••••`                            |
| `PORT`                   | Web server port                              | `8100`                               |
| `RUST_LOG`               | Log level and tracing filters                | `debug,reqwest=trace,hyper=trace`    |

---

## 🚀 Running Locally

```bash
cargo run
```

The app listens on `http://0.0.0.0:$PORT` (defaults to `8100`).

Test the health of the service:

```bash
curl http://localhost:8100/api/health
```

Expected output:

```json
{ "ok": true, "status": "ready" }
```

---

## ☁️ Deploying to Render

1. **Add Environment Variables**
   Under your Render service → **Environment → Environment Variables**, define all variables listed above.

2. **Specify build command**

   ```
   cargo build --release
   ```

3. **Specify start command**

   ```
   ./target/release/suppression-policy-engine
   ```

4. **Port**
   Ensure your Render service listens on `$PORT` (Render automatically injects it).

---

## 🔐 ControlByWeb Integration

- The `ControlByWebDriver` handles all device communication.
- For each apply request:
  1. Resolves `(account_id, device_id)` for the given installation.
  2. Authenticates via OAuth.
  3. **Creates a DAT (Device Access Token)** valid for 5 minutes.
  4. **Constructs a URL** like:
     ```
     https://productionblue.api.controlbyweb.cloud/DAT/{DAT}/customState.json?x21Relay3=1&x19Relay4=1...
     ```
  5. Executes relay changes (`1` = ON, `0` = OFF).
  6. Deletes the DAT afterward to keep within ControlByWeb’s limit of 3 active DATs per device.

Relay states can be verified via the ControlByWeb dashboard:
👉 [Device Portal](https://api.controlbyweb.cloud/accounts/3023095475/devs/2168150121/setup.html#)

---

## 🧠 Example

```rust
use suppression_policy_engine::device_abstraction_layer::drivers::control_by_web::relay_plan::plan_relays;
use suppression_policy_engine::device_abstraction_layer::Command;

let plan = plan_relays("sebastians_house", Command::Lockdown);
println!("Turning ON: {:?}", plan.on);
```

Output:

```text
Turning ON: ["x21Relay1", "x21Relay2", ..., "x19Relay16"]
```

---

## 📚 Related Docs

- [`docs/controlbyweb_driver.md`](docs/controlbyweb_driver.md) — ControlByWeb authentication, DAT lifecycle, and relay plan.
- [`docs/architecture.md`](docs/architecture.md) — Overview of SPE architecture, module dependencies, and control flow.
