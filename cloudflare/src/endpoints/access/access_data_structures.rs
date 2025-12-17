use crate::framework::response::ApiResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Access Application
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/applications/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AccessApplication {
    /// Application identifier
    pub id: String,
    /// Application name
    pub name: String,
    /// Application domain
    pub domain: String,
    /// Application type
    #[serde(rename = "type")]
    pub application_type: ApplicationType,
    /// Application session duration in hours
    pub session_duration: Option<String>,
    /// Auto redirect to identity provider
    pub auto_redirect_to_identity: Option<bool>,
    /// Allowed domains for CORS
    pub allowed_idps: Option<Vec<String>>,
    /// CORS settings
    pub cors_headers: Option<CorsHeaders>,
    /// Custom deny message
    pub custom_deny_message: Option<String>,
    /// Custom deny URL
    pub custom_deny_url: Option<String>,
    /// Custom pages
    pub custom_pages: Option<serde_json::Value>,
    /// Application tags
    pub tags: Option<Vec<String>>,
    /// Application logo URL
    pub logo_url: Option<String>,
    /// Whether to skip the identity provider selection page
    pub skip_interstitial: Option<bool>,
    /// Application creation date
    pub created_at: Option<DateTime<Utc>>,
    /// Application last update date
    pub updated_at: Option<DateTime<Utc>>,
    /// Application policies
    pub policies: Option<Vec<String>>,
}

/// Application types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationType {
    /// Self-hosted application
    SelfHosted,
    /// SSH application  
    Ssh,
    /// VNC application
    Vnc,
    /// File sharing application
    Biso,
    /// Application container
    AppLauncher,
    /// Bookmark application
    Bookmark,
    /// SaaS application
    Saas,
}

/// CORS headers configuration
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CorsHeaders {
    /// Allowed origins
    pub allowed_origins: Option<Vec<String>>,
    /// Allowed methods
    pub allowed_methods: Option<Vec<String>>,
    /// Allowed headers
    pub allowed_headers: Option<Vec<String>>,
    /// Allow credentials
    pub allow_credentials: Option<bool>,
    /// Max age for preflight cache
    pub max_age: Option<u32>,
}

/// Access Policy
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/policies/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AccessPolicy {
    /// Policy identifier
    pub id: String,
    /// Policy name
    pub name: String,
    /// Policy decision (allow, deny, non_identity, bypass)
    pub decision: PolicyDecision,
    /// Rules that must be satisfied
    pub include: Vec<AccessRule>,
    /// Rules that must not be satisfied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<AccessRule>>,
    /// Additional rules that must be satisfied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<Vec<AccessRule>>,
    /// Approval groups for the policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_groups: Option<Vec<ApprovalGroup>>,
    /// Session duration override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
    /// Policy creation date
    pub created_at: Option<DateTime<Utc>>,
    /// Policy last update date
    pub updated_at: Option<DateTime<Utc>>,
}

/// Policy decisions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    /// Allow access
    Allow,
    /// Deny access
    Deny,
    /// Non-identity decision
    NonIdentity,
    /// Bypass authentication
    Bypass,
}

/// Access Rule
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "rule_type", rename_all = "snake_case")]
pub enum AccessRule {
    /// Email rule
    Email {
        /// Email address
        email: String,
    },
    /// Email domain rule
    EmailDomain {
        /// Email domain
        email_domain: String,
    },
    /// IP rule
    Ip {
        /// IP address or CIDR
        ip: String,
    },
    /// Country rule
    Country {
        /// Country code
        country: String,
    },
    /// Access group rule
    Group {
        /// Access group ID
        group: String,
    },
    /// Service token rule
    ServiceToken {
        /// Service token ID
        service_token: String,
    },
    /// Everyone rule
    Everyone {},
    /// Custom rule with certificate
    Certificate {},
    /// Azure group rule
    AzureGroup {
        /// Azure group ID
        azure_group: String,
    },
    /// GitHub organization rule
    GitHubOrganization {
        /// GitHub organization name
        github_organization: String,
    },
    /// Google Workspace group rule
    GoogleWorkspaceGroup {
        /// Google Workspace group email
        google_workspace_group: String,
    },
}

/// Approval Group
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ApprovalGroup {
    /// Number of approvals required
    pub approvals_needed: u32,
    /// Email addresses of approvers
    pub email_addresses: Option<Vec<String>>,
    /// List of email address lists for approvals
    pub email_list_uuid: Option<String>,
}

/// Access User
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AccessUser {
    /// User identifier
    pub id: String,
    /// User email address
    pub email: String,
    /// User name
    pub name: Option<String>,
    /// User creation date
    pub created_at: Option<DateTime<Utc>>,
    /// User last update date
    pub updated_at: Option<DateTime<Utc>>,
    /// Last successful authentication
    pub last_successful_login: Option<DateTime<Utc>>,
}

/// Service Token
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServiceToken {
    /// Token identifier
    pub id: String,
    /// Token name
    pub name: String,
    /// Client ID for the service token
    pub client_id: String,
    /// Client secret (only returned when creating)
    pub client_secret: Option<String>,
    /// Token expiration date
    pub expires_at: Option<DateTime<Utc>>,
    /// Token creation date
    pub created_at: Option<DateTime<Utc>>,
    /// Token last update date
    pub updated_at: Option<DateTime<Utc>>,
    /// Last time token was used
    pub last_used_at: Option<DateTime<Utc>>,
    /// Token duration in minutes
    pub duration: Option<String>,
}

/// Parameters for creating an Access application
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateAccessApplicationParams {
    /// Application name
    pub name: String,
    /// Application domain
    pub domain: String,
    /// Application type
    #[serde(rename = "type")]
    pub application_type: ApplicationType,
    /// Session duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
    /// Auto redirect to identity provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redirect_to_identity: Option<bool>,
    /// Allowed identity providers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_idps: Option<Vec<String>>,
    /// CORS settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_headers: Option<CorsHeaders>,
    /// Custom deny message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_deny_message: Option<String>,
    /// Custom deny URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_deny_url: Option<String>,
    /// Application tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Application logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Skip interstitial page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_interstitial: Option<bool>,
}

/// Parameters for updating an Access application
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateAccessApplicationParams {
    /// Application name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Session duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
    /// Auto redirect to identity provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redirect_to_identity: Option<bool>,
    /// Allowed identity providers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_idps: Option<Vec<String>>,
    /// CORS settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_headers: Option<CorsHeaders>,
    /// Custom deny message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_deny_message: Option<String>,
    /// Custom deny URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_deny_url: Option<String>,
    /// Application tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Application logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Skip interstitial page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_interstitial: Option<bool>,
}

/// Parameters for creating an Access policy
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateAccessPolicyParams {
    /// Policy name
    pub name: String,
    /// Policy decision
    pub decision: PolicyDecision,
    /// Include rules
    pub include: Vec<AccessRule>,
    /// Exclude rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<AccessRule>>,
    /// Require rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<Vec<AccessRule>>,
    /// Approval groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_groups: Option<Vec<ApprovalGroup>>,
    /// Session duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
}

/// Parameters for updating an Access policy
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateAccessPolicyParams {
    /// Policy name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Policy decision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<PolicyDecision>,
    /// Include rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<AccessRule>>,
    /// Exclude rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<AccessRule>>,
    /// Require rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<Vec<AccessRule>>,
    /// Approval groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_groups: Option<Vec<ApprovalGroup>>,
    /// Session duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<String>,
}

/// Parameters for creating a service token
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateServiceTokenParams {
    /// Token name
    pub name: String,
    /// Token duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

/// Parameters for updating a service token
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateServiceTokenParams {
    /// Token name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Token duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

// ApiResult trait implementations
impl ApiResult for AccessApplication {}
impl ApiResult for Vec<AccessApplication> {}
impl ApiResult for AccessPolicy {}
impl ApiResult for Vec<AccessPolicy> {}
impl ApiResult for AccessUser {}
impl ApiResult for Vec<AccessUser> {}
impl ApiResult for ServiceToken {}
impl ApiResult for Vec<ServiceToken> {}