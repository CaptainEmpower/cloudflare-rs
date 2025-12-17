use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::access_data_structures::{
    AccessApplication, CreateAccessApplicationParams, UpdateAccessApplicationParams,
};

/// List Access applications
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/applications/methods/list/
#[derive(Debug)]
pub struct ListAccessApplications<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for ListAccessApplications<'_> {
    type JsonResponse = Vec<AccessApplication>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        if self.is_account {
            format!("accounts/{}/access/apps", self.account_id)
        } else {
            format!("zones/{}/access/apps", self.account_id)
        }
    }
}

/// Get an Access application
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/applications/methods/get/
#[derive(Debug)]
pub struct GetAccessApplication<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Application identifier
    pub app_id: &'a str,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for GetAccessApplication<'_> {
    type JsonResponse = AccessApplication;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        if self.is_account {
            format!("accounts/{}/access/apps/{}", self.account_id, self.app_id)
        } else {
            format!("zones/{}/access/apps/{}", self.account_id, self.app_id)
        }
    }
}

/// Create an Access application
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/applications/methods/create/
#[derive(Debug)]
pub struct CreateAccessApplication<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Application creation parameters
    pub params: CreateAccessApplicationParams,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for CreateAccessApplication<'_> {
    type JsonResponse = AccessApplication;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        if self.is_account {
            format!("accounts/{}/access/apps", self.account_id)
        } else {
            format!("zones/{}/access/apps", self.account_id)
        }
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Update an Access application
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/applications/methods/update/
#[derive(Debug)]
pub struct UpdateAccessApplication<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Application identifier
    pub app_id: &'a str,
    /// Application update parameters
    pub params: UpdateAccessApplicationParams,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for UpdateAccessApplication<'_> {
    type JsonResponse = AccessApplication;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        if self.is_account {
            format!("accounts/{}/access/apps/{}", self.account_id, self.app_id)
        } else {
            format!("zones/{}/access/apps/{}", self.account_id, self.app_id)
        }
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Delete an Access application
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/applications/methods/delete/
#[derive(Debug)]
pub struct DeleteAccessApplication<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Application identifier
    pub app_id: &'a str,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for DeleteAccessApplication<'_> {
    type JsonResponse = serde_json::Value; // Returns {"id": "app_id"}
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        if self.is_account {
            format!("accounts/{}/access/apps/{}", self.account_id, self.app_id)
        } else {
            format!("zones/{}/access/apps/{}", self.account_id, self.app_id)
        }
    }
}