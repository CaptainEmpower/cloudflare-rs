use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::ssl_data_structures::{
    CreateOriginCaCertificateParams, DeleteCertificateResponse, OriginCaCertificate,
    UpdateSslSettingsParams, ZoneSslSettings,
};

/// List Origin CA certificates
/// https://developers.cloudflare.com/api/resources/origin_ca_certificates/methods/list/
#[derive(Debug)]
pub struct ListOriginCaCertificates {
    /// Zone identifier (optional, for zone-level certificates)
    pub zone_id: Option<String>,
}

impl EndpointSpec for ListOriginCaCertificates {
    type JsonResponse = Vec<OriginCaCertificate>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "certificates".to_string()
    }

    fn query(&self) -> Option<String> {
        self.zone_id
            .as_ref()
            .map(|zone_id| format!("zone_id={}", zone_id))
    }
}

/// Get an Origin CA certificate by ID
/// https://developers.cloudflare.com/api/resources/origin_ca_certificates/methods/get/
#[derive(Debug)]
pub struct GetOriginCaCertificate<'a> {
    /// Certificate identifier
    pub certificate_id: &'a str,
}

impl EndpointSpec for GetOriginCaCertificate<'_> {
    type JsonResponse = OriginCaCertificate;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("certificates/{}", self.certificate_id)
    }
}

/// Create a new Origin CA certificate
/// https://developers.cloudflare.com/api/resources/origin_ca_certificates/methods/create/
#[derive(Debug)]
pub struct CreateOriginCaCertificate {
    /// Certificate creation parameters
    pub params: CreateOriginCaCertificateParams,
}

impl EndpointSpec for CreateOriginCaCertificate {
    type JsonResponse = OriginCaCertificate;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "certificates".to_string()
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Revoke an Origin CA certificate
/// https://developers.cloudflare.com/api/resources/origin_ca_certificates/methods/revoke/
#[derive(Debug)]
pub struct RevokeOriginCaCertificate<'a> {
    /// Certificate identifier
    pub certificate_id: &'a str,
}

impl EndpointSpec for RevokeOriginCaCertificate<'_> {
    type JsonResponse = DeleteCertificateResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("certificates/{}", self.certificate_id)
    }
}

/// Get SSL settings for a zone
/// https://developers.cloudflare.com/api/resources/zone_settings/subresources/ssl/methods/get/
#[derive(Debug)]
pub struct GetZoneSslSettings<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
}

impl EndpointSpec for GetZoneSslSettings<'_> {
    type JsonResponse = ZoneSslSettings;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("zones/{}/settings/ssl", self.zone_id)
    }
}

/// Update SSL settings for a zone
/// https://developers.cloudflare.com/api/resources/zone_settings/subresources/ssl/methods/edit/
#[derive(Debug)]
pub struct UpdateZoneSslSettings<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
    /// SSL settings update parameters
    pub params: UpdateSslSettingsParams,
}

impl EndpointSpec for UpdateZoneSslSettings<'_> {
    type JsonResponse = ZoneSslSettings;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!("zones/{}/settings/ssl", self.zone_id)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}