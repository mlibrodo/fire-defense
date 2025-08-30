use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

pub mod error;
pub mod types;

use error::IrwinError;

// Re-export types for public API
pub use types::{IrwinConfig, IncidentQuery, IncidentQueryBuilder};
pub use types::environments;

/// A stateless client for the IRWIN Incidents API
pub struct IrwinClient {
    config: IrwinConfig,
    http_client: Client,
}

impl IrwinClient {
    /// Create a new IRWIN client with the given configuration
    pub fn new(config: IrwinConfig) -> Result<Self, IrwinError> {
        let mut headers = HeaderMap::new();
        headers.insert("Referer", HeaderValue::from_str(&config.referer)?);
        
        let http_client = Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Get the API version information
    pub async fn get_api_version(&self) -> Result<String, IrwinError> {
        let url = format!("{}/info?f=json", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;
        
        if response.status().is_success() {
            let text = response.text().await?;
            Ok(text)
        } else {
            Err(IrwinError::ApiError {
                status: response.status().as_u16(),
                message: format!("Failed to get API version: {}", response.status()),
            })
        }
    }

    /// Query incidents using the provided query parameters
    pub async fn query_incidents(&self, query: &IncidentQuery) -> Result<String, IrwinError> {
        let mut params = Vec::new();
        
        // Add required parameters
        params.push(("f".to_string(), "json".to_string()));
        
        // Add query parameters
        if let Some(where_clause) = &query.where_clause {
            params.push(("where".to_string(), where_clause.clone()));
        }
        
        if let Some(out_fields) = &query.out_fields {
            params.push(("outFields".to_string(), out_fields.clone()));
        }
        
        if let Some(return_geometry) = &query.return_geometry {
            params.push(("returnGeometry".to_string(), return_geometry.clone()));
        }
        
        // Add IRWIN-specific extensions
        if query.include_ads_status {
            params.push(("includeADSStatus".to_string(), "true".to_string()));
        }
        
        if query.include_resources {
            params.push(("includeResources".to_string(), "true".to_string()));
        }
        
        if query.include_relationships {
            params.push(("includeRelationships".to_string(), "true".to_string()));
        }
        
        if query.include_last_sync_date_time {
            params.push(("includeLastSyncDateTime".to_string(), "true".to_string()));
        }
        
        if query.include_ffr {
            params.push(("includeFFR".to_string(), "true".to_string()));
        }
        
        // Generate token and add to params
        let token = self.generate_token().await?;
        params.push(("token".to_string(), token));
        
        // Build URL with query parameters
        let mut url = format!("{}/Irwin/Incidents/FeatureServer/0/query", self.config.base_url);
        let query_string: String = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        url.push_str(&format!("?{}", query_string));
        
        // Make the request
        let response = self.http_client.get(&url).send().await?;
        
        if response.status().is_success() {
            let text = response.text().await?;
            Ok(text)
        } else {
            Err(IrwinError::ApiError {
                status: response.status().as_u16(),
                message: format!("Failed to query incidents: {}", response.status()),
            })
        }
    }

    /// Query incidents by IRWIN IDs
    pub async fn query_by_irwin_ids(&self, irwin_ids: &[String]) -> Result<String, IrwinError> {
        let where_clause = format!("IrwinID IN ({})", irwin_ids.join(","));
        let query = IncidentQueryBuilder::new()
            .where_clause(&where_clause)
            .build();
        
        self.query_incidents(&query).await
    }

    /// Query incidents by unique fire identifiers
    pub async fn query_by_unique_fire_identifiers(&self, fire_ids: &[String]) -> Result<String, IrwinError> {
        let where_clause = format!("UniqueFireIdentifier IN ({})", fire_ids.join(","));
        let query = IncidentQueryBuilder::new()
            .where_clause(&where_clause)
            .build();
        
        self.query_incidents(&query).await
    }

    /// Sync incidents since a specific timestamp
    pub async fn sync_since(&self, timestamp: u64) -> Result<String, IrwinError> {
        let where_clause = format!("ModifiedOnDateTime >= {}", timestamp);
        let query = IncidentQueryBuilder::new()
            .where_clause(&where_clause)
            .include_last_sync_date_time(true)
            .build();
        
        self.query_incidents(&query).await
    }

    /// Get current timestamp in milliseconds
    pub fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Generate an authentication token
    async fn generate_token(&self) -> Result<String, IrwinError> {
        let mut params = Vec::new();
        params.push(("username".to_string(), self.config.username.clone()));
        params.push(("password".to_string(), self.config.password.clone()));
        params.push(("client".to_string(), "referer".to_string()));
        params.push(("referer".to_string(), self.config.referer.clone()));
        params.push(("expiration".to_string(), "60".to_string()));
        params.push(("f".to_string(), "json".to_string()));
        
        let url = format!("{}/tokens/generateToken", self.config.base_url);
        
        // Build form data manually since we don't have the form feature
        let body = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        
        let response = self.http_client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await?;
        
        if response.status().is_success() {
            let text = response.text().await?;
            // Parse the JSON response to extract the token
            let token_response: serde_json::Value = serde_json::from_str(&text)?;
            
            if let Some(token) = token_response["token"].as_str() {
                Ok(token.to_string())
            } else {
                Err(IrwinError::ApiError {
                    status: 0,
                    message: "No token in response".to_string(),
                })
            }
        } else {
            Err(IrwinError::ApiError {
                status: response.status().as_u16(),
                message: format!("Failed to generate token: {}", response.status()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::environments;

    #[test]
    fn test_current_timestamp_ms() {
        let timestamp = IrwinClient::current_timestamp_ms();
        assert!(timestamp > 0);
    }

    #[test]
    fn test_incident_query_builder() {
        let query = IncidentQueryBuilder::new()
            .where_clause("IsValid=1 AND IncidentTypeKind='FI'")
            .out_fields("IrwinID,IncidentName,UniqueFireIdentifier")
            .return_geometry("true")
            .include_resources(true)
            .include_relationships(true)
            .build();
        
        assert_eq!(query.where_clause, Some("IsValid=1 AND IncidentTypeKind='FI'".to_string()));
        assert_eq!(query.out_fields, Some("IrwinID,IncidentName,UniqueFireIdentifier".to_string()));
        assert_eq!(query.return_geometry, Some("true".to_string()));
        assert!(query.include_resources);
        assert!(query.include_relationships);
    }
}
