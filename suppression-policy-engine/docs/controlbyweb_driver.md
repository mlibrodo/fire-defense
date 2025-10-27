# ⚙️ ControlByWeb Driver

The **ControlByWeb driver** implements the `DeviceDriver` trait for physical device control.
It encapsulates the full lifecycle of **authenticating**, **creating temporary DATs**, and **executing relay updates**.

---

## 🔑 Authentication Flow

1. Authenticate via OAuth password grant:

   ```http
   POST /api/v1/auth/token
   grant_type=password
   username={CBW_USERNAME}
   password={CBW_PASSWORD}
   ```

2. Obtain an access token for the account.

3. Use this token for all subsequent account-scoped requests.

---

## 🧩 Device Access Token (DAT) Lifecycle

Each command execution:

1. **Lists existing DATs**
   `GET /v1/accounts/{AccountId}/devices/{DeviceId}/DAT`

2. **Creates a new DAT** if needed
   `POST /v1/accounts/{AccountId}/devices/{DeviceId}/DAT`
   with `minutesValid=5`

3. **Applies command using DAT URL**

   ```
   GET https://productionblue.api.controlbyweb.cloud/DAT/{DAT}/customState.json?x21Relay1=1&x19Relay3=0...
   ```

4. **Deletes DAT after use**
   ```
   DELETE /v1/accounts/{AccountId}/devices/{DeviceId}/DAT/{DAT}
   ```

---

## 🔦 Relay Mapping

Relays are defined in `relay_plan.rs`:

| Command              | Relays ON            | Description          |
| -------------------- | -------------------- | -------------------- |
| `Monitor`            | none                 | Idle                 |
| `ArmSensors`         | x21Relay1            | Activate sensor      |
| `EnablePumpsLow`     | x21Relay3, x19Relay4 | Moderate defense     |
| `EnablePumpsHigh`    | x21Relay3–x19Relay6  | Full pump            |
| `OpenValvesPriority` | x19Relay11–12        | Prioritized flow     |
| `OpenValvesAll`      | x19Relay11–16        | All zones open       |
| `Lockdown`           | _All relays ON_      | Maximum defense mode |

Relay state changes can be viewed on:
👉 [ControlByWeb Portal](https://api.controlbyweb.cloud/accounts/3023095475/devs/2168150121/setup.html#)

---

## 🧠 Implementation Notes

- The driver wraps a `TokenManager` in an `Arc<Mutex<>>` since it’s shared across async tasks.
- `DeviceAccessTokenManager` is stateless: it creates DATs per use (no caching).
- Uses `.with_device_access(...)` to automatically clean up DATs after use (like a context manager).
