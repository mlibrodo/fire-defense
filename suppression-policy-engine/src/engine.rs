use crate::policy::Policy;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Evaluation {
    pub ok: bool,
    pub installation_id: String,
    pub policy: Policy,
    pub level: u8,
    pub actions: Vec<String>,
    pub dry_run: bool,
}

#[derive(Debug, Default)]
pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, installation_id: &str, policy: Policy, dry_run: bool) -> Evaluation {
        let actions = policy.actions().iter().map(|s| s.to_string()).collect();
        Evaluation {
            ok: true,
            installation_id: installation_id.to_string(),
            policy,
            level: policy.level(),
            actions,
            dry_run,
        }
    }
}
