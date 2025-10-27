use crate::device_abstraction_layer::Command;

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
    "x19Relay11",
    "x19Relay12",
    "x19Relay13",
    "x19Relay14",
    "x19Relay15",
    "x19Relay16",
];

/// Result of planning relays for a command.
#[derive(Debug, Clone)]
pub struct RelayPlan {
    pub on: Vec<&'static str>,
    pub off: Vec<&'static str>,
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

    // Compute off = ALL_RELAYS - on
    let on_set = on_slice
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let off = ALL_RELAYS
        .iter()
        .copied()
        .filter(|k| !on_set.contains(k))
        .collect::<Vec<_>>();

    RelayPlan {
        on: on_slice.to_vec(),
        off,
    }
}
