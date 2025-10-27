use anyhow::Result;
use async_trait::async_trait;

use super::{Command, CommandResult};

#[async_trait]
pub trait DeviceDriver: Send + Sync {
    async fn apply(&self, installation_id: &str, cmd: Command) -> Result<CommandResult>;

    async fn status(&self, installation_id: &str) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"ok": true, "installation_id": installation_id}))
    }
}
