use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::access_data_structures::{
    CreateServiceTokenParams, ServiceToken, UpdateServiceTokenParams,
};

/// List Access service tokens
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/service_tokens/methods/list/
#[derive(Debug)]
pub struct ListAccessServiceTokens<'a> {
    /// Account identifier
    pub account_id: &'a str,
}

impl EndpointSpec for ListAccessServiceTokens<'_> {
    type JsonResponse = Vec<ServiceToken>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/access/service_tokens", self.account_id)
    }
}

/// Get an Access service token
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/service_tokens/methods/get/
#[derive(Debug)]
pub struct GetAccessServiceToken<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Service token identifier
    pub token_id: &'a str,
}

impl EndpointSpec for GetAccessServiceToken<'_> {
    type JsonResponse = ServiceToken;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/access/service_tokens/{}",
            self.account_id, self.token_id
        )
    }
}

/// Create an Access service token
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/service_tokens/methods/create/
#[derive(Debug)]
pub struct CreateAccessServiceToken<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Service token creation parameters
    pub params: CreateServiceTokenParams,
}

impl EndpointSpec for CreateAccessServiceToken<'_> {
    type JsonResponse = ServiceToken;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("accounts/{}/access/service_tokens", self.account_id)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Update an Access service token
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/service_tokens/methods/update/
#[derive(Debug)]
pub struct UpdateAccessServiceToken<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Service token identifier
    pub token_id: &'a str,
    /// Service token update parameters
    pub params: UpdateServiceTokenParams,
}

impl EndpointSpec for UpdateAccessServiceToken<'_> {
    type JsonResponse = ServiceToken;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/access/service_tokens/{}",
            self.account_id, self.token_id
        )
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Delete an Access service token
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/service_tokens/methods/delete/
#[derive(Debug)]
pub struct DeleteAccessServiceToken<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Service token identifier
    pub token_id: &'a str,
}

impl EndpointSpec for DeleteAccessServiceToken<'_> {
    type JsonResponse = serde_json::Value; // Returns {"id": "token_id"}
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/access/service_tokens/{}",
            self.account_id, self.token_id
        )
    }
}

/// Rotate an Access service token
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/service_tokens/methods/rotate/
#[derive(Debug)]
pub struct RotateAccessServiceToken<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Service token identifier
    pub token_id: &'a str,
}

impl EndpointSpec for RotateAccessServiceToken<'_> {
    type JsonResponse = ServiceToken;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/access/service_tokens/{}/rotate",
            self.account_id, self.token_id
        )
    }
}