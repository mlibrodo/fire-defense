use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::{Client, Url};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error};

use super::{
    config::ControlByWebConfig, device_access::DeviceAccessTokenManager, token::TokenManager,
};
use crate::device_abstraction_layer::drivers::control_by_web::account::InstallationAccountResolver;
use crate::device_abstraction_layer::drivers::control_by_web::relay_plan::{
    plan_relays, RelayPlan,
};
use crate::device_abstraction_layer::{Command, CommandResult, DeviceDriver};

/// Base host for direct-to-device DAT calls.
/// If you want this configurable, move to config and pass through `ControlByWebConfig`.
const DAT_BASE: &str = "https://productionblue.api.controlbyweb.cloud/";

#[derive(Clone)]
pub struct ControlByWebDriver {
    client: Client,
    base_url: Url,
    tm: Arc<Mutex<TokenManager>>,
    dats: DeviceAccessTokenManager,
    resolver: Arc<dyn InstallationAccountResolver>,
}

impl ControlByWebDriver {
    pub fn new(
        cfg: ControlByWebConfig,
        resolver: Arc<dyn InstallationAccountResolver>,
    ) -> Result<Self> {
        let client = cfg.build_client()?;
        let token_url = cfg.token_url()?;
        let base_url = cfg.base_url.clone();
        let tm = Arc::new(Mutex::new(TokenManager::new(
            client.clone(),
            token_url,
            cfg.username,
            cfg.password,
        )));
        let dats = DeviceAccessTokenManager::new(client.clone(), cfg.base_url.clone(), tm.clone());

        Ok(Self {
            client,
            base_url,
            tm,
            dats,
            resolver,
        })
    }

    #[inline]
    fn _endpoint(&self, rel: &str) -> Result<Url> {
        Ok(self.base_url.join(rel)?)
    }
}

/// Build: https://.../DAT/{dat}/customState.json?KEY=1&...&KEY=0...
fn build_dat_custom_state_url(dat: &str, plan: &RelayPlan) -> Result<Url> {
    let base = Url::parse(DAT_BASE)?;
    let mut url = base.join(&format!("DAT/{}/customState.json", dat))?;
    {
        let mut qp = url.query_pairs_mut();
        for k in &plan.on {
            qp.append_pair(k, "1");
        }
        for k in &plan.off {
            qp.append_pair(k, "0");
        }
    }
    Ok(url)
}

#[async_trait]
impl DeviceDriver for ControlByWebDriver {
    async fn apply(&self, installation_id: &str, cmd: Command) -> Result<CommandResult> {
        // 1) Resolve (account_id, device_id)
        let binding = self.resolver.resolve(installation_id).ok_or_else(|| {
            anyhow::anyhow!("no account/device binding for installation {installation_id}")
        })?;
        let account_id = binding.account_id;
        let device_id = binding.device_id; // String (owned)
        debug!("apply(): installation_id={installation_id} account_id={account_id} device_id={device_id:?} cmd={:?}", cmd);

        // 2) Plan relays once, return ON and OFF lists
        let plan = plan_relays(installation_id, cmd);
        debug!("relay plan: ON={:?} OFF={:?}", plan.on, plan.off);

        // 3) Acquire a short-lived DAT; call device; delete if created
        let minutes_valid: u32 = 5;
        let client = self.client.clone();

        self.dats
            .with_device_access(account_id, &device_id, minutes_valid, move |dat_ref| {
                let dat = dat_ref.to_string();
                let plan = plan.clone();
                let client = client.clone();

                async move {
                    let url = build_dat_custom_state_url(&dat, &plan)
                        .context("building DAT customState url")?;
                    debug!("DAT customState url={url}");

                    // Direct device call — DAT URL doesn’t need OAuth bearer
                    let resp = client
                        .get(url.clone())
                        .send()
                        .await
                        .context("sending DAT customState request")?;

                    if !resp.status().is_success() {
                        let code = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        error!("device DAT not ok: status={} body={}", code, text);
                        anyhow::bail!("device error via DAT: status={} body={}", code, text);
                    }

                    // (Optional) inspect body:
                    // let _body = resp.text().await.unwrap_or_default();

                    Ok::<CommandResult, anyhow::Error>(CommandResult {
                        ok: true,
                        message: "relays updated".into(),
                    })
                }
            })
            .await
    }

    async fn status(&self, installation_id: &str) -> Result<serde_json::Value> {
        // Stub for now
        Ok(json!({ "ok": true, "installation_id": installation_id }))
    }
}
