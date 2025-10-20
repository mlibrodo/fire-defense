use crate::dal::{Command, CommandResult, DeviceDriver};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ControlByWebDriver {
    pub base_url: String,
    pub api_key: Option<String>,
    // add client, timeouts, etc.
}

#[async_trait]
impl DeviceDriver for ControlByWebDriver {
    async fn apply(&self, installation_id: &str, cmd: Command) -> anyhow::Result<CommandResult> {
        // TODO: implement actual HTTP/IO
        let _ = (installation_id, cmd);
        Ok(CommandResult {
            ok: true,
            message: "stub".into(),
        })
    }
}
