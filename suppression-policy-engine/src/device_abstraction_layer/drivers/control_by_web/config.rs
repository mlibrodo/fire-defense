use reqwest::Client;
use reqwest::Url;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ControlByWebConfig {
    pub base_url: Url,
    pub username: String,
    pub password: String,
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
}

impl ControlByWebConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let base =
            std::env::var("CBW_BASE_URL").map_err(|_| anyhow::anyhow!("CBW_BASE_URL not set"))?;
        let username =
            std::env::var("CBW_USERNAME").map_err(|_| anyhow::anyhow!("CBW_USERNAME not set"))?;
        let password =
            std::env::var("CBW_PASSWORD").map_err(|_| anyhow::anyhow!("CBW_PASSWORD not set"))?;

        Ok(Self {
            base_url: Url::parse(&base)?,
            username,
            password,
            connect_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(8),
        })
    }

    pub fn build_client(&self) -> anyhow::Result<Client> {
        Ok(Client::builder()
            .use_rustls_tls()
            .connect_timeout(self.connect_timeout)
            .timeout(self.request_timeout)
            .build()?)
    }

    pub fn token_url(&self) -> anyhow::Result<Url> {
        Ok(self.base_url.join("/api/v1/auth/token")?)
    }
}
