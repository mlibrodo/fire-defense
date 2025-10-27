# 🧭 Architecture Overview

The **Suppression Policy Engine (SPE)** manages the lifecycle of wildfire defense automation.

---

## 🏗 Major Modules

| Module                         | Responsibility                                                 |
| ------------------------------ | -------------------------------------------------------------- |
| `policy.rs`                    | Defines high-level suppression policies (Observe → Suppress)   |
| `engine.rs`                    | Evaluates installation + policy → determines level and actions |
| `suppression_policy_runner.rs` | Executes evaluated actions asynchronously                      |
| `device_abstraction_layer/`    | Abstracts control across hardware backends                     |
| `drivers/control_by_web/`      | Actual hardware integration layer                              |
| `web/`                         | Exposes HTTP API for evaluation and execution                  |
| `state/`                       | Holds shared app state between components                      |

---

## 🔁 Control Flow

```
HTTP Request (POST /api/evaluate)
        │
        ▼
  Engine::evaluate()
        │
        ├─ Determines policy level
        ├─ Builds Evaluation {installation_id, policy, actions}
        ▼
  Runner::apply()
        │
        ├─ Looks up installation via resolver
        ├─ Calls DAL (ControlByWebDriver)
        │     ├─ Authenticates via TokenManager
        │     ├─ Creates temporary DAT
        │     ├─ Builds relay plan
        │     ├─ Calls customState.json
        │     └─ Deletes DAT
        ▼
   Returns CommandResult
```

---

## 🧩 Engine vs Runner

| Component | Role                                         | Output       |
| --------- | -------------------------------------------- | ------------ |
| `Engine`  | Evaluates a given policy for an installation | `Evaluation` |
| `Runner`  | Executes the `Evaluation` (via DAL)          | `RunResult`  |

---

## 🔌 Device Abstraction Layer (DAL)

The DAL provides a unified interface across all device integrations:

```rust
#[async_trait]
pub trait DeviceDriver {
    async fn apply(&self, installation_id: &str, cmd: Command) -> Result<CommandResult>;
    async fn status(&self, installation_id: &str) -> Result<serde_json::Value>;
}
```

Concrete implementations:

- `ControlByWebDriver`
- `MockDriver` (for testing)

---

## 🔍 Installation Resolution

Each `installation_id` maps to a specific `(account_id, device_id)` via the
`InMemoryResolver`, allowing multiple installations per ControlByWeb account.

Example:

```rust
map.insert("sebastians_house".to_string(), (3023095475, "2168150121".to_string()));
```

---

## 🌐 HTTP API

| Endpoint             | Description                         |
| -------------------- | ----------------------------------- |
| `POST /api/evaluate` | Evaluate a policy (dry-run or real) |
| `POST /api/run`      | Execute evaluation results          |
| `GET /api/health`    | Service health check                |

---

## 🪶 Future Work

- 🔄 Persistent resolver (database-backed)
- 🧠 Policy tuning via external triggers (e.g., weather/wind)
- 📊 Telemetry pipeline (event + metrics reporting)
- 🧩 Additional drivers (e.g., Modbus, MQTT)
