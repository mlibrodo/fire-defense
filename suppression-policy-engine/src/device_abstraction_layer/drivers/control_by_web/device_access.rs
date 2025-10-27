use super::token::TokenManager;
use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Url};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, warn};

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceAccessTokenItem {
    pub token: String,
}

#[derive(Clone)]
pub struct DeviceAccessTokenManager {
    client: Client,
    base_url: Url,
    tm: Arc<Mutex<TokenManager>>,
}

impl DeviceAccessTokenManager {
    pub fn new(client: Client, base_url: Url, tm: Arc<Mutex<TokenManager>>) -> Self {
        Self {
            client,
            base_url,
            tm,
        }
    }

    #[inline]
    fn list_url(&self, account_id: u64, device_id: &str) -> Result<Url> {
        Ok(self.base_url.join(&format!(
            "api/v1/accounts/{account_id}/devices/{device_id}/DAT"
        ))?)
    }

    #[inline]
    fn create_url(&self, account_id: u64, device_id: &str) -> Result<Url> {
        // Per docs, POST to the same path with form minutesValid
        Ok(self.base_url.join(&format!(
            "api/v1/accounts/{account_id}/devices/{device_id}/DAT"
        ))?)
    }

    #[inline]
    fn delete_url(&self, account_id: u64, device_id: &str, dat: &str) -> Result<Url> {
        Ok(self.base_url.join(&format!(
            "api/v1/accounts/{account_id}/devices/{device_id}/DAT/{dat}"
        ))?)
    }

    /// GET list of DATs for a device. Returns an array of items.
    pub async fn list_tokens(
        &self,
        account_id: u64,
        device_id: &str,
    ) -> Result<Vec<DeviceAccessTokenItem>> {
        let url = self.list_url(account_id, device_id)?;
        debug!("List DATs url={url}");
        let mut req = self.client.get(url.clone());
        {
            let mut tm = self.tm.lock().await;
            req = tm.attach_auth(req).await?;
        }
        let resp = req
            .send()
            .await
            .context("DAT list send failed")?
            .error_for_status()
            .context("DAT list non-2xx")?;
        let body = resp.bytes().await.context("reading DAT list body")?;
        let items: Vec<DeviceAccessTokenItem> = serde_json::from_slice(&body).map_err(|e| {
            anyhow!(
                "parse DAT list JSON failed: {e}; body={}",
                String::from_utf8_lossy(&body)
            )
        })?;
        Ok(items)
    }

    /// POST create a DAT (returns {"message":"success"}). You must list again to get the token.
    pub async fn create_token(
        &self,
        account_id: u64,
        device_id: &str,
        minutes_valid: u32,
    ) -> Result<()> {
        let url = self.create_url(account_id, device_id)?;
        debug!("Create DAT url={url} minutes_valid={minutes_valid}");
        let mut req = self
            .client
            .post(url.clone())
            .form(&[("minutesValid", minutes_valid.to_string())]);
        {
            let mut tm = self.tm.lock().await;
            req = tm.attach_auth(req).await?;
        }
        let resp = req
            .send()
            .await
            .context("DAT create send failed")?
            .error_for_status()
            .context("DAT create non-2xx")?;
        let body = resp.bytes().await.unwrap_or_default();
        debug!(
            "DAT create response len={} body={}",
            body.len(),
            String::from_utf8_lossy(&body)
        );
        Ok(())
    }

    /// DELETE a specific DAT
    pub async fn delete_token(&self, account_id: u64, device_id: &str, dat: &str) -> Result<()> {
        let url = self.delete_url(account_id, device_id, dat)?;
        debug!("Delete DAT url={url}");
        let mut req = self.client.delete(url.clone());
        {
            let mut tm = self.tm.lock().await;
            req = tm.attach_auth(req).await?;
        }
        let resp = req
            .send()
            .await
            .context("DAT delete send failed")?
            .error_for_status()
            .context("DAT delete non-2xx")?;
        let body = resp.bytes().await.unwrap_or_default();
        debug!(
            "DAT delete response len={} body={}",
            body.len(),
            String::from_utf8_lossy(&body)
        );
        Ok(())
    }

    /// Ensure a usable token for (account, device) without caching:
    /// - If any tokens exist, just return one (prefers the last element).
    /// - Otherwise, create one (minutes_valid) and list again; return the newest token.
    ///
    /// Returns (token, created_new).
    pub async fn ensure_device_access_token(
        &self,
        account_id: u64,
        device_id: &str,
        minutes_valid: u32,
    ) -> Result<(String, bool)> {
        let before = self.list_tokens(account_id, device_id).await?;
        if let Some(tok) = before.last().map(|i| i.token.clone()) {
            return Ok((tok, false));
        }

        // none exist — create and list again
        self.create_token(account_id, device_id, minutes_valid)
            .await?;
        let after = self.list_tokens(account_id, device_id).await?;
        if let Some(tok) = after.last().map(|i| i.token.clone()) {
            return Ok((tok, true));
        }

        Err(anyhow!(
            "DAT create succeeded but no token found after listing"
        ))
    }

    /// Context-manager style helper: create or pick a DAT, run `f(dat)`, then delete it if we created it.
    ///
    /// Usage:
    /// device_tokens.with_device_access(aid, did, 5, |dat| async move {
    ///     // use `dat` here (e.g., build /DAT/{dat}/state.json URL)
    ///     Ok(())
    /// }).await?;
    pub async fn with_device_access<F, Fut, T>(
        &self,
        account_id: u64,
        device_id: &str,
        minutes_valid: u32,
        f: F,
    ) -> Result<T>
    where
        F: FnOnce(&str) -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let (dat, created) = self
            .ensure_device_access_token(account_id, device_id, minutes_valid)
            .await?;
        let res = f(&dat).await;
        if created {
            if let Err(e) = self.delete_token(account_id, device_id, &dat).await {
                warn!(
                    "cleanup: failed to delete DAT {} for device {}: {:?}",
                    dat, device_id, e
                );
            }
        }
        res
    }
}
