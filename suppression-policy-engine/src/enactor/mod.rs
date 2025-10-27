// src/enactor/mod.rs
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::device_abstraction_layer::{Command, CommandResult, DeviceDriver};
use crate::policy::Policy;

#[derive(Debug, Clone)]
pub struct EnactStep {
    pub name: String,
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct EnactReport {
    pub policy: Policy,
    pub steps: Vec<EnactStep>,
    pub ok: bool,
}

#[async_trait]
pub trait InstallationEnactor: Send + Sync {
    /// Translate policy → device operations and execute them (unless dry_run)
    async fn enact(
        &self,
        installation_id: &str,
        policy: Policy,
        dry_run: bool,
    ) -> Result<EnactReport>;
}

/// Simple mapper that uses your existing DeviceDriver::apply(Command)
pub struct SimpleEnactor {
    driver: Arc<dyn DeviceDriver>,
}

impl SimpleEnactor {
    pub fn new(driver: Arc<dyn DeviceDriver>) -> Self {
        Self { driver }
    }

    /// Hide the mapping here: policy → commands. Not exposed to engine/policy.
    fn plan(policy: Policy) -> Vec<Command> {
        match policy {
            Policy::Observe => vec![Command::Monitor],
            Policy::Prepare => vec![Command::ArmSensors, Command::StagePumps],
            Policy::Defend => vec![Command::ArmSensors, Command::EnablePumpsLow],
            Policy::Contain => vec![
                Command::ArmSensors,
                Command::EnablePumpsHigh,
                Command::OpenValvesPriority,
            ],
            Policy::Suppress => vec![Command::Lockdown],
            Policy::Unknown => vec![Command::Noop],
        }
    }
}

#[async_trait]
impl InstallationEnactor for SimpleEnactor {
    async fn enact(
        &self,
        installation_id: &str,
        policy: Policy,
        dry_run: bool,
    ) -> Result<EnactReport> {
        let mut steps = Vec::new();
        let mut all_ok = true;

        for cmd in Self::plan(policy) {
            if dry_run {
                steps.push(EnactStep {
                    name: format!("{cmd:?}"),
                    ok: true,
                    message: "dry_run".into(),
                });
                continue;
            }

            // Execute via DAL
            let res: Result<CommandResult> = self.driver.apply(installation_id, cmd.clone()).await;
            match res {
                Ok(r) if r.ok => steps.push(EnactStep {
                    name: format!("{cmd:?}"),
                    ok: true,
                    message: r.message,
                }),
                Ok(r) => {
                    all_ok = false;
                    steps.push(EnactStep {
                        name: format!("{cmd:?}"),
                        ok: false,
                        message: r.message,
                    });
                    break;
                }
                Err(e) => {
                    all_ok = false;
                    steps.push(EnactStep {
                        name: format!("{cmd:?}"),
                        ok: false,
                        message: e.to_string(),
                    });
                    break;
                }
            }
        }

        Ok(EnactReport {
            policy,
            steps,
            ok: all_ok,
        })
    }
}
