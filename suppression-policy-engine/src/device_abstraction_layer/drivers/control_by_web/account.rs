use std::collections::HashMap;
use tracing::debug;

/// Represents the resolved mapping for an installation
#[derive(Debug, Clone)]
pub struct AccountBinding {
    pub account_id: u64,
    pub device_id: String,
}

pub trait InstallationAccountResolver: Send + Sync {
    /// Returns both account_id and device_id for a given installation_id
    fn resolve(&self, installation_id: &str) -> Option<AccountBinding>;
}

/// Simple in-memory implementation
pub struct InMemoryResolver {
    /// installation_id -> (account_id, device_id)
    map: HashMap<String, (u64, String)>,
    /// optional default fallback
    default_account: Option<u64>,
    default_device: Option<String>,
}

impl InMemoryResolver {
    pub fn new(
        map: HashMap<String, (u64, String)>,
        default_account: Option<u64>,
        default_device: Option<String>,
    ) -> Self {
        Self {
            map,
            default_account,
            default_device,
        }
    }
}

impl InstallationAccountResolver for InMemoryResolver {
    fn resolve(&self, installation_id: &str) -> Option<AccountBinding> {
        if let Some((account_id, device_id)) = self.map.get(installation_id) {
            Some(AccountBinding {
                account_id: *account_id,
                device_id: device_id.clone(),
            })
        } else if let (Some(aid), Some(did)) = (self.default_account, &self.default_device) {
            debug!(
                "Using default device id {} and device id {} for installation {}",
                aid, did, installation_id
            );
            Some(AccountBinding {
                account_id: aid,
                device_id: did.clone(),
            })
        } else {
            None
        }
    }
}
