use crate::device_abstraction_layer::Command;
use anyhow::Context;
use serde_json::Value;
use std::collections::HashMap;

/// All relay keys we care about (from your sample payload).
/// Anything not returned in `on` will be listed in `off`.
pub const ALL_RELAYS: &[&str] = &[
    // x21
    "x21Relay1",
    "x21Relay2",
    "x21Relay3",
    "x21Relay4",
    // x19 (1..8, 11..16 as shown in your JSON)
    "x19Relay1",
    "x19Relay2",
    "x19Relay3",
    "x19Relay4",
    "x19Relay5",
    "x19Relay6",
    "x19Relay7",
    "x19Relay8",
    "x19Relay10",
    "x19Relay11",
    "x19Relay12",
    "x19Relay13",
    "x19Relay14",
    "x19Relay15",
    "x19Relay16",
];

#[derive(Debug, Clone)]
pub struct RelayPlan {
    pub on: Vec<String>,
    pub off: Vec<String>,
}

/// Map (installation_id, Command) → which relays should be **ON**.
/// Everything else from `ALL_RELAYS` goes to **OFF**.
pub fn plan_relays(_installation_id: &str, cmd: Command) -> RelayPlan {
    // Hardcoded mapping for now — adjust as your policies evolve.
    let on_slice: &'static [&'static str] = match cmd {
        Command::Monitor => &[], // nothing on
        Command::ArmSensors => &["x21Relay1"],
        Command::EnablePumpsLow => &["x21Relay3", "x19Relay4"],
        Command::EnablePumpsHigh => &["x21Relay3", "x19Relay4", "x19Relay5", "x19Relay6"],
        Command::OpenValvesPriority => &["x19Relay11", "x19Relay12"],
        Command::OpenValvesAll => &[
            "x19Relay11",
            "x19Relay12",
            "x19Relay13",
            "x19Relay14",
            "x19Relay15",
            "x19Relay16",
        ],
        Command::Lockdown => ALL_RELAYS,
        _ => &[],
    };

    let on: Vec<String> = on_slice.iter().map(|&s| s.to_string()).collect();
    let off: Vec<String> = ALL_RELAYS
        .iter()
        .copied()
        .filter(|k| !on_slice.contains(k))
        .map(|s| s.to_string())
        .collect();

    RelayPlan { on, off }
}

/// Parse a batch customState body into relay -> bool map
pub fn parse_relay_state_map(body: &str) -> anyhow::Result<HashMap<String, bool>> {
    let v: Value = serde_json::from_str(body).with_context(|| {
        format!(
            "parsing batch customState JSON: {}",
            &body[..body.len().min(400)]
        )
    })?;
    let obj = v
        .as_object()
        .context("customState JSON was not an object")?;

    let mut out = HashMap::new();
    for (k, val) in obj {
        // Only consider relay keys we care about
        let is_relay = k.starts_with("x21Relay") || k.starts_with("x19Relay");
        if !is_relay {
            continue;
        }

        // Values can be "1 @1.3" (string) or sometimes numeric 0/1
        let on = if let Some(s) = val.as_str() {
            // Take the part before the first space or '@'
            let cut = s.split(|c| c == ' ' || c == '@').next().unwrap_or("");
            cut == "1" || cut == "true"
        } else if let Some(n) = val.as_u64() {
            n == 1
        } else if let Some(b) = val.as_bool() {
            b
        } else {
            // Unknown shape, ignore this key
            continue;
        };

        out.insert(k.clone(), on);
    }
    Ok(out)
}

/// Build expected relay map from the plan
pub fn expected_from_plan(plan: &RelayPlan) -> HashMap<String, bool> {
    let mut m = HashMap::new();
    for r in &plan.on {
        m.insert(r.to_string(), true);
    }
    for r in &plan.off {
        m.insert(r.to_string(), false);
    }
    m
}

/// Compute mismatches (relays that didn't end up with desired state)
pub fn mismatches(
    expected: &HashMap<String, bool>,
    actual: &HashMap<String, bool>,
) -> Vec<(String, bool, Option<bool>)> {
    let mut diffs = Vec::new();
    for (relay, &want_on) in expected {
        let got = actual.get(relay).copied();
        if got != Some(want_on) {
            diffs.push((relay.clone(), want_on, got));
        }
    }
    diffs
}
