use serde::{Deserialize, Serialize};
use std::fmt;

/// Five-level policy scale with Unknown catch-all for extensibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Policy {
    Observe,  // L1
    Prepare,  // L2
    Defend,   // L3
    Contain,  // L4
    Suppress, // L5

    /// Accept unknown strings without failing deserialization.
    #[serde(other)]
    Unknown,
}

impl Policy {
    pub fn level(self) -> u8 {
        match self {
            Policy::Observe => 1,
            Policy::Prepare => 2,
            Policy::Defend => 3,
            Policy::Contain => 4,
            Policy::Suppress => 5,
            Policy::Unknown => 0,
        }
    }

    pub fn summary(self) -> &'static str {
        match self {
            Policy::Observe => "Monitor conditions; no active defenses.",
            Policy::Prepare => "Arm sensors; stage equipment.",
            Policy::Defend => "Activate low-intensity defenses.",
            Policy::Contain => "High-intensity defenses; prioritize containment.",
            Policy::Suppress => "Maximum response; all systems engaged.",
            Policy::Unknown => "Unrecognized policy.",
        }
    }

    pub fn actions(self) -> &'static [&'static str] {
        match self {
            Policy::Observe => &["monitor"],
            Policy::Prepare => &["arm_sensors", "stage_pumps"],
            Policy::Defend => &["arm_sensors", "enable_pumps_low"],
            Policy::Contain => &["arm_sensors", "enable_pumps_high", "open_valves_priority"],
            Policy::Suppress => &[
                "arm_sensors",
                "enable_pumps_high",
                "open_valves_all",
                "lockdown",
            ],
            Policy::Unknown => &["noop"],
        }
    }
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Policy::Observe => "observe",
            Policy::Prepare => "prepare",
            Policy::Defend => "defend",
            Policy::Contain => "contain",
            Policy::Suppress => "suppress",
            Policy::Unknown => "unknown",
        })
    }
}
