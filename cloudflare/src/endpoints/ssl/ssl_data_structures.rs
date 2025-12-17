use crate::framework::response::ApiResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Origin CA Certificate
/// https://developers.cloudflare.com/api/resources/origin_ca_certificates/
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OriginCaCertificate {
    /// Certificate identifier
    pub id: String,
    /// The Certificate Signing Request (CSR)
    pub csr: String,
    /// List of hostnames or wildcard names bound to the certificate
    pub hostnames: Vec<String>,
    /// The date the certificate expires
    pub expires_on: DateTime<Utc>,
    /// The signature type of the certificate
    pub request_type: String,
    /// The certificate authority that will issue the certificate
    pub certificate_authority: Option<String>,
    /// The generated certificate
    pub certificate: String,
    /// The type of hash used for the certificate
    pub signature: Option<String>,
    /// Certificate creation date
    pub created_on: Option<DateTime<Utc>>,
    /// Certificate modification date
    pub modified_on: Option<DateTime<Utc>>,
    /// Certificate status
    pub status: Option<String>,
}

/// Parameters for creating an Origin CA certificate
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateOriginCaCertificateParams {
    /// The Certificate Signing Request (CSR). Must be newline-encoded
    pub csr: String,
    /// Array of hostnames or wildcard names (e.g., *.example.com) bound to the certificate
    pub hostnames: Vec<String>,
    /// Signature type desired on certificate (origin-rsa, origin-ecc, keyless-certificate)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    /// The number of days for which the certificate should be valid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_validity: Option<u32>,
}

/// Certificate request types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CertificateRequestType {
    /// RSA signature type
    OriginRsa,
    /// ECC signature type  
    OriginEcc,
    /// Keyless certificate for Keyless SSL servers
    KeylessCertificate,
}

impl std::fmt::Display for CertificateRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CertificateRequestType::OriginRsa => write!(f, "origin-rsa"),
            CertificateRequestType::OriginEcc => write!(f, "origin-ecc"),
            CertificateRequestType::KeylessCertificate => write!(f, "keyless-certificate"),
        }
    }
}

/// Common validity periods for certificates (in days)
pub struct CertificateValidity;

impl CertificateValidity {
    /// 7 days validity
    pub const WEEK: u32 = 7;
    /// 30 days validity (1 month)
    pub const MONTH: u32 = 30;
    /// 90 days validity (3 months)
    pub const THREE_MONTHS: u32 = 90;
    /// 365 days validity (1 year)
    pub const YEAR: u32 = 365;
    /// 730 days validity (2 years)
    pub const TWO_YEARS: u32 = 730;
    /// 5475 days validity (15 years, maximum)
    pub const FIFTEEN_YEARS: u32 = 5475;
}

/// Response for certificate deletion
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeleteCertificateResponse {
    /// Certificate identifier that was deleted
    pub id: String,
}

/// Certificate validation error
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CertificateValidationError {
    /// Error code
    pub code: Option<String>,
    /// Error message
    pub message: String,
    /// Field that caused the error
    pub field: Option<String>,
}

impl CreateOriginCaCertificateParams {
    /// Create certificate parameters with RSA signature
    pub fn new_rsa(csr: String, hostnames: Vec<String>) -> Self {
        Self {
            csr,
            hostnames,
            request_type: Some(CertificateRequestType::OriginRsa.to_string()),
            requested_validity: Some(CertificateValidity::YEAR),
        }
    }

    /// Create certificate parameters with ECC signature
    pub fn new_ecc(csr: String, hostnames: Vec<String>) -> Self {
        Self {
            csr,
            hostnames,
            request_type: Some(CertificateRequestType::OriginEcc.to_string()),
            requested_validity: Some(CertificateValidity::YEAR),
        }
    }

    /// Create certificate parameters with custom validity period
    pub fn with_validity(mut self, days: u32) -> Self {
        self.requested_validity = Some(days);
        self
    }

    /// Create certificate parameters for keyless certificate
    pub fn new_keyless(csr: String, hostnames: Vec<String>) -> Self {
        Self {
            csr,
            hostnames,
            request_type: Some(CertificateRequestType::KeylessCertificate.to_string()),
            requested_validity: Some(CertificateValidity::YEAR),
        }
    }
}

/// SSL/TLS Settings for a Zone
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ZoneSslSettings {
    /// SSL mode for the zone
    pub value: SslMode,
    /// Last modified timestamp
    pub modified_on: Option<DateTime<Utc>>,
    /// Whether the setting is editable
    pub editable: Option<bool>,
}

/// SSL/TLS Mode
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SslMode {
    /// No SSL
    Off,
    /// Flexible SSL
    Flexible,
    /// Full SSL
    Full,
    /// Full SSL (Strict)
    FullStrict,
}

/// Parameters for updating SSL settings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UpdateSslSettingsParams {
    /// SSL mode to set
    pub value: SslMode,
}

// ApiResult trait implementations
impl ApiResult for OriginCaCertificate {}
impl ApiResult for Vec<OriginCaCertificate> {}
impl ApiResult for DeleteCertificateResponse {}
impl ApiResult for ZoneSslSettings {}