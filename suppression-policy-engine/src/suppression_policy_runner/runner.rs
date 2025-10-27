use tracing::{error, info};

use crate::state::AppState;
use crate::{models::RunStatus, time::now_ms};

pub fn spawn_run(state: AppState, run_id: String) {
    tokio::spawn(async move {
        // load run
        let (installation_id, policy, dry_run) = {
            let g = state.runs.read().await;
            if let Some(r) = g.get(&run_id) {
                (r.installation_id.clone(), r.policy, r.dry_run)
            } else {
                return;
            }
        };

        {
            let mut w = state.runs.write().await;
            if let Some(r) = w.get_mut(&run_id) {
                r.status = RunStatus::Running;
                r.updated_at_ms = now_ms();
            }
        }

        match state.enactor.enact(&installation_id, policy, dry_run).await {
            Ok(report) if report.ok => {
                info!(%run_id, ?policy, "run succeeded");
                let mut w = state.runs.write().await;
                if let Some(r) = w.get_mut(&run_id) {
                    r.status = RunStatus::Succeeded;
                    r.updated_at_ms = now_ms();
                }
            }
            Ok(report) => {
                error!(%run_id, ?policy, steps=?report.steps, "run failed");
                let mut w = state.runs.write().await;
                if let Some(r) = w.get_mut(&run_id) {
                    r.status = RunStatus::Failed;
                    r.updated_at_ms = now_ms();
                }
            }
            Err(e) => {
                error!(%run_id, error=%e, "enactor error");
                let mut w = state.runs.write().await;
                if let Some(r) = w.get_mut(&run_id) {
                    r.status = RunStatus::Failed;
                    r.updated_at_ms = now_ms();
                }
            }
        }
    });
}
