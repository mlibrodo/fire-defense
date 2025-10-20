use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TelemetryEvent<'a> {
    pub installation_id: &'a str,
    pub run_id: Option<&'a str>,
    pub kind: &'a str,
    pub data: serde_json::Value,
}

#[async_trait]
pub trait TelemetrySink: Send + Sync {
    async fn send(&self, ev: TelemetryEvent<'_>);
}

pub struct NoopSink;
#[async_trait]
impl TelemetrySink for NoopSink {
    async fn send(&self, _ev: TelemetryEvent<'_>) {}
}
