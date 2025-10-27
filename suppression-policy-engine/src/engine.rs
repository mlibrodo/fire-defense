use crate::policy::Policy;
use serde::Serialize;

/// A pure, side-effect-free evaluation of policy intent for an installation.
/// The engine does not prescribe device actions; it summarizes intent/level.
/// Execution is handled by the enactor + DAL.
#[derive(Debug, Serialize)]
pub struct Evaluation {
    pub ok: bool,
    pub installation_id: String,
    pub policy: Policy,
    pub level: u8,
    pub summary: &'static str,
    pub dry_run: bool,
}

#[derive(Debug, Default)]
pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    /// Plan at the intent level (no device actions here).
    /// Return the policy level and a human-readable summary.
    pub fn evaluate(&self, installation_id: &str, policy: Policy, dry_run: bool) -> Evaluation {
        Evaluation {
            ok: true,
            installation_id: installation_id.to_string(),
            policy,
            level: policy.level(),
            summary: policy.summary(),
            dry_run,
        }
    }
}
