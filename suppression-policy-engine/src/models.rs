use crate::policy::Policy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Starting,
    Running,
    Succeeded,
    Failed,
    Canceling,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub run_id: String,
    pub installation_id: String,
    pub policy: Policy,
    pub level: u8,
    pub actions: Vec<String>,
    pub dry_run: bool,
    pub status: RunStatus,
    pub started_at_ms: u128,
    pub updated_at_ms: u128,

    // internal flags (not strictly needed yet)
    #[serde(skip)]
    pub cancel_requested: bool,
}

#[derive(Debug, Deserialize)]
pub struct StartRunRequest {
    pub policy: Policy,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub policy: Policy,
    #[serde(default)]
    pub dry_run: bool,
    pub installation_id: Option<String>,
}
