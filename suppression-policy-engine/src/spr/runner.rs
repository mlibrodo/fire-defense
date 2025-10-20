use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};
use tracing::{error, info};

use crate::{
    dal::{Command, DeviceDriver},
    models::{RunRecord, RunStatus},
    time::now_ms,
};

fn action_to_command(s: &str) -> Command {
    match s {
        "monitor" => Command::Monitor,
        "arm_sensors" => Command::ArmSensors,
        "stage_pumps" => Command::StagePumps,
        "enable_pumps_low" => Command::EnablePumpsLow,
        "enable_pumps_high" => Command::EnablePumpsHigh,
        "open_valves_priority" => Command::OpenValvesPriority,
        "open_valves_all" => Command::OpenValvesAll,
        "lockdown" => Command::Lockdown,
        _ => Command::Noop,
    }
}

pub fn spawn_run(
    runs: Arc<RwLock<HashMap<String, RunRecord>>>,
    dal: Arc<dyn DeviceDriver>,
    run_id: String,
) {
    tokio::spawn(async move {
        {
            let mut g = runs.write().await;
            if let Some(r) = g.get_mut(&run_id) {
                r.status = RunStatus::Running;
                r.updated_at_ms = now_ms();
            } else {
                return;
            }
        }

        let actions = {
            let g = runs.read().await;
            g.get(&run_id)
                .map(|r| r.actions.clone())
                .unwrap_or_default()
        };

        for a in actions {
            // cancellation?
            {
                let g = runs.read().await;
                if let Some(r) = g.get(&run_id) {
                    if r.cancel_requested {
                        drop(g);
                        let mut w = runs.write().await;
                        if let Some(r2) = w.get_mut(&run_id) {
                            r2.status = RunStatus::Canceled;
                            r2.updated_at_ms = now_ms();
                        }
                        info!(%run_id, "run canceled");
                        return;
                    }
                } else {
                    return;
                }
            }

            // apply via DAL
            let installation_id = {
                let g = runs.read().await;
                g.get(&run_id)
                    .map(|r| r.installation_id.clone())
                    .unwrap_or_default()
            };
            let cmd = action_to_command(&a);
            match dal.apply(&installation_id, cmd).await {
                Ok(res) if res.ok => info!(%run_id, ?cmd, "command ok"),
                Ok(res) => {
                    error!(%run_id, ?cmd, ?res, "command not ok");
                    let mut w = runs.write().await;
                    if let Some(r) = w.get_mut(&run_id) {
                        r.status = RunStatus::Failed;
                        r.updated_at_ms = now_ms();
                    }
                    return;
                }
                Err(e) => {
                    error!(%run_id, ?cmd, error=%e, "command error");
                    let mut w = runs.write().await;
                    if let Some(r) = w.get_mut(&run_id) {
                        r.status = RunStatus::Failed;
                        r.updated_at_ms = now_ms();
                    }
                    return;
                }
            }

            sleep(Duration::from_millis(150)).await;
        }

        let mut w = runs.write().await;
        if let Some(r) = w.get_mut(&run_id) {
            r.status = RunStatus::Succeeded;
            r.updated_at_ms = now_ms();
        }
        info!(%run_id, "run succeeded");
    });
}
