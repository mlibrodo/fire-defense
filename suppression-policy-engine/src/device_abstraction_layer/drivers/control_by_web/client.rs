use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::{Client, Url};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error};

use super::{
    config::ControlByWebConfig, device_access::DeviceAccessTokenManager,
    relay_plan::parse_relay_state_map, token::TokenManager,
};
use crate::device_abstraction_layer::drivers::control_by_web::account::InstallationAccountResolver;
use crate::device_abstraction_layer::drivers::control_by_web::relay_plan::{
    expected_from_plan, mismatches, plan_relays, RelayPlan,
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
                    // 1) Batch attempt
                    let url = build_dat_custom_state_url(&dat, &plan)
                        .context("building DAT customState url")?;
                    debug!("DAT customState url={url}");

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

                    let body = resp.text().await.unwrap_or_default();
                    debug!("Batch response: {}", &body[..body.len().min(400)]);

                    // 2) Compare plan vs actual
                    let actual = parse_relay_state_map(&body).unwrap_or_default();
                    let expected = expected_from_plan(&plan);
                    let diffs = mismatches(&expected, &actual);

                    if diffs.is_empty() {
                        // Batch matched the plan — great!
                        return Ok::<CommandResult, anyhow::Error>(CommandResult {
                            ok: true,
                            message: "relays updated (batch)".into(),
                        });
                    }

                    // 3) Fallback: fix mismatches individually
                    debug!(
                        "Batch mismatch; falling back to per-relay updates: {:?}",
                        diffs
                    );
                    let mut successes = Vec::new();
                    let mut failures = Vec::new();

                    for (relay, want_on, got) in diffs {
                        let single_url =
                            build_single_dat_url(&dat, &relay, want_on).with_context(|| {
                                format!(
                                    "build single relay url for {}={}",
                                    relay,
                                    if want_on { 1 } else { 0 }
                                )
                            })?;

                        let r = client
                            .get(single_url.clone())
                            .send()
                            .await
                            .with_context(|| {
                                format!(
                                    "send single relay {}={}",
                                    relay,
                                    if want_on { 1 } else { 0 }
                                )
                            })?;

                        if !r.status().is_success() {
                            let code = r.status();
                            let text = r.text().await.unwrap_or_default();
                            error!(
                                "single relay not ok: {} wanted={} got={:?} status={} body={}",
                                relay, want_on, got, code, text
                            );
                            failures.push((relay, want_on, code.as_u16(), text));
                            // If you want to stop on first failure, break here.
                            continue;
                        }

                        // (Optional) read body for logging
                        let b = r.text().await.unwrap_or_default();
                        debug!(
                            "single relay OK: {}={} body={}",
                            relay,
                            if want_on { 1 } else { 0 },
                            b
                        );
                        successes.push((relay, want_on));
                    }

                    // 4) Finalize
                    if failures.is_empty() {
                        return Ok::<CommandResult, anyhow::Error>(CommandResult {
                            ok: true,
                            message: format!(
                                "relays updated (fallback ok, {} fixed)",
                                successes.len()
                            ),
                        });
                    } else {
                        let summary = serde_json::to_string(&failures).unwrap_or_default();
                        anyhow::bail!("fallback failed for some relays: {}", summary);
                    }
                }
            })
            .await
    }

    async fn status(&self, installation_id: &str) -> Result<serde_json::Value> {
        // Stub for now
        Ok(json!({ "ok": true, "installation_id": installation_id }))
    }
}

/// Build a one-relay DAT URL by reusing your existing batch builder
fn build_single_dat_url(dat: &str, relay: &str, on: bool) -> anyhow::Result<reqwest::Url> {
    let one = RelayPlan {
        on: if on { vec![relay.to_string()] } else { vec![] },
        off: if on { vec![] } else { vec![relay.to_string()] },
    };
    build_dat_custom_state_url(dat, &one)
}
