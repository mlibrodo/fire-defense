use anyhow::Context;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::time::{Duration, Instant};
#[derive(Debug, Deserialize)]
pub struct TokenResp {
    pub expires_in: u64,
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
}
use tracing::debug;

pub struct TokenManager {
    client: Client,
    auth_token_url: Url,
    username: String,
    password: String,

    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<Instant>,
}

impl TokenManager {
    pub fn new(client: Client, auth_token_url: Url, username: String, password: String) -> Self {
        Self {
            client,
            auth_token_url,
            username,
            password,
            access_token: None,
            refresh_token: None,
            expires_at: None,
        }
    }

    pub async fn ensure_token(&mut self) -> anyhow::Result<String> {
        debug!("Ensuring token");
        if let (Some(tok), Some(exp)) = (&self.access_token, self.expires_at) {
            if Instant::now() + Duration::from_secs(60) < exp {
                return Ok(tok.clone());
            }
        }
        if let Some(rt) = self.refresh_token.clone() {
            if let Ok(tok) = self.refresh(rt).await {
                return Ok(tok);
            }
        }
        self.login_password().await
    }

    async fn login_password(&mut self) -> anyhow::Result<String> {
        debug!("Logging password");
        let resp = self
            .client
            .post(self.auth_token_url.clone())
            .form(&[
                ("grant_type", "password"),
                ("username", self.username.as_str()),
                ("password", self.password.as_str()),
            ])
            .send()
            .await
            .context("token request (password grant) failed to send")?
            .error_for_status()
            .context("token request (password grant) returned non-2xx")?;

        // Read once, then you can log & parse
        let body = resp.bytes().await.context("reading token body")?;
        debug!(
            "token request (password grant) succeeded using {}",
            self.auth_token_url.as_str()
        );

        let tr: TokenResp = serde_json::from_slice(&body).map_err(|e| {
            anyhow::anyhow!(
                "parse token JSON failed: {e}; body={}",
                String::from_utf8_lossy(&body)
            )
        })?;

        debug!("setting token for login");
        self.set_tokens(tr);
        Ok(self.access_token.clone().unwrap())
    }

    async fn refresh(&mut self, refresh_token: String) -> anyhow::Result<String> {
        let resp = self
            .client
            .post(self.auth_token_url.clone())
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token.as_str()),
            ])
            .send()
            .await
            .context("token request (refresh grant) failed to send")?
            .error_for_status()
            .context("token request (refresh grant) returned non-2xx")?;

        let body = resp.bytes().await.context("reading refresh body")?;
        debug!(
            "token request (refresh grant) succeeded using {}",
            self.auth_token_url.as_str()
        );

        let tr: TokenResp = serde_json::from_slice(&body).map_err(|e| {
            anyhow::anyhow!(
                "parse refresh JSON failed: {e}; body={}",
                String::from_utf8_lossy(&body)
            )
        })?;

        debug!("setting token for refresh");
        self.set_tokens(tr);
        Ok(self.access_token.clone().unwrap())
    }

    fn set_tokens(&mut self, tr: TokenResp) {
        self.access_token = Some(tr.access_token);
        self.refresh_token = tr.refresh_token;
        let exp = Instant::now() + Duration::from_secs(tr.expires_in.saturating_sub(60));
        self.expires_at = Some(exp);
    }

    pub async fn attach_auth(
        &mut self,
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::RequestBuilder> {
        debug!("attaching auth {req:#?}");
        let tok = self.ensure_token().await?;
        Ok(req.bearer_auth(tok))
    }
}
