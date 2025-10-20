use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Monitor,
    ArmSensors,
    StagePumps,
    EnablePumpsLow,
    EnablePumpsHigh,
    OpenValvesPriority,
    OpenValvesAll,
    Lockdown,
    Noop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub ok: bool,
    pub message: String,
}
