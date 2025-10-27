use async_trait::async_trait;
use tracing::info;

use crate::device_abstraction_layer::{Command, CommandResult, DeviceDriver};

pub struct MockDriver;

#[async_trait]
impl DeviceDriver for MockDriver {
    async fn apply(&self, installation_id: &str, cmd: Command) -> anyhow::Result<CommandResult> {
        info!(%installation_id, ?cmd, "MockDriver.apply");
        Ok(CommandResult {
            ok: true,
            message: format!("applied {:?}", cmd),
        })
    }
}
