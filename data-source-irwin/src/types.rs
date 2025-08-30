use std::time::Duration;

/// Configuration for the IRWIN client
#[derive(Clone, Debug)]
pub struct IrwinConfig {
    /// Base URL for the IRWIN API (e.g., https://irwin.doi.gov/arcgis/rest/services)
    pub base_url: String,
    
    /// Username for authentication
    pub username: String,
    
    /// Password for authentication
    pub password: String,
    
    /// Referer string for authentication
    pub referer: String,
    
    /// Request timeout
    pub timeout: Duration,
}

impl IrwinConfig {
    /// Create a new configuration with default timeout
    pub fn new(base_url: String, username: String, password: String, referer: String) -> Self {
        Self {
            base_url,
            username,
            password,
            referer,
            timeout: Duration::from_secs(30),
        }
    }
    
    /// Create a new configuration with custom timeout
    pub fn with_timeout(
        base_url: String,
        username: String,
        password: String,
        referer: String,
        timeout: Duration,
    ) -> Self {
        Self {
            base_url,
            username,
            password,
            referer,
            timeout,
        }
    }
}

/// Query parameters for incident queries
#[derive(Clone, Debug)]
pub struct IncidentQuery {
    /// WHERE clause for filtering incidents
    pub where_clause: Option<String>,
    
    /// Comma-separated list of output fields
    pub out_fields: Option<String>,
    
    /// Whether to return geometry
    pub return_geometry: Option<String>,
    
    /// Include ADS status
    pub include_ads_status: bool,
    
    /// Include resources
    pub include_resources: bool,
    
    /// Include relationships
    pub include_relationships: bool,
    
    /// Include last sync date time
    pub include_last_sync_date_time: bool,
    
    /// Include FFR (Fire Funding Request)
    pub include_ffr: bool,
}

/// Builder for IncidentQuery
pub struct IncidentQueryBuilder {
    pub where_clause: Option<String>,
    pub out_fields: Option<String>,
    pub return_geometry: Option<String>,
    pub include_ads_status: bool,
    pub include_resources: bool,
    pub include_relationships: bool,
    pub include_last_sync_date_time: bool,
    pub include_ffr: bool,
}

impl IncidentQueryBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            where_clause: None,
            out_fields: None,
            return_geometry: None,
            include_ads_status: false,
            include_resources: false,
            include_relationships: false,
            include_last_sync_date_time: false,
            include_ffr: false,
        }
    }
    
    /// Set the WHERE clause
    pub fn where_clause(mut self, where_clause: &str) -> Self {
        self.where_clause = Some(where_clause.to_string());
        self
    }
    
    /// Set the output fields
    pub fn out_fields(mut self, out_fields: &str) -> Self {
        self.out_fields = Some(out_fields.to_string());
        self
    }
    
    /// Set whether to return geometry
    pub fn return_geometry(mut self, return_geometry: &str) -> Self {
        self.return_geometry = Some(return_geometry.to_string());
        self
    }
    
    /// Set whether to include ADS status
    pub fn include_ads_status(mut self, include: bool) -> Self {
        self.include_ads_status = include;
        self
    }
    
    /// Set whether to include resources
    pub fn include_resources(mut self, include: bool) -> Self {
        self.include_resources = include;
        self
    }
    
    /// Set whether to include relationships
    pub fn include_relationships(mut self, include: bool) -> Self {
        self.include_relationships = include;
        self
    }
    
    /// Set whether to include last sync date time
    pub fn include_last_sync_date_time(mut self, include: bool) -> Self {
        self.include_last_sync_date_time = include;
        self
    }
    
    /// Set whether to include FFR
    pub fn include_ffr(mut self, include: bool) -> Self {
        self.include_ffr = include;
        self
    }
    
    /// Build the IncidentQuery
    pub fn build(self) -> IncidentQuery {
        IncidentQuery {
            where_clause: self.where_clause,
            out_fields: self.out_fields,
            return_geometry: self.return_geometry,
            include_ads_status: self.include_ads_status,
            include_resources: self.include_resources,
            include_relationships: self.include_relationships,
            include_last_sync_date_time: self.include_last_sync_date_time,
            include_ffr: self.include_ffr,
        }
    }
}

impl Default for IncidentQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// IRWIN environment URLs
pub mod environments {
    /// Test environment URL
    pub const TEST: &str = "https://irwint.doi.gov/arcgis/rest/services";
    
    /// OAT (Operational Acceptance Testing) environment URL
    pub const OAT: &str = "https://irwinoat.doi.gov/arcgis/rest/services";
    
    /// Production environment URL
    pub const PROD: &str = "https://irwin.doi.gov/arcgis/rest/services";
}
