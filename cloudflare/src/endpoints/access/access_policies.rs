use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::access_data_structures::{AccessPolicy, CreateAccessPolicyParams, UpdateAccessPolicyParams};

/// List Access policies for an application
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/policies/methods/list/
#[derive(Debug)]
pub struct ListAccessPolicies<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Application identifier (optional for account-level policies)
    pub app_id: Option<&'a str>,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for ListAccessPolicies<'_> {
    type JsonResponse = Vec<AccessPolicy>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        let base = if self.is_account {
            format!("accounts/{}", self.account_id)
        } else {
            format!("zones/{}", self.account_id)
        };

        match self.app_id {
            Some(app_id) => format!("{}/access/apps/{}/policies", base, app_id),
            None => format!("{}/access/policies", base),
        }
    }
}

/// Get an Access policy
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/policies/methods/get/
#[derive(Debug)]
pub struct GetAccessPolicy<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Policy identifier
    pub policy_id: &'a str,
    /// Application identifier (optional for account-level policies)
    pub app_id: Option<&'a str>,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for GetAccessPolicy<'_> {
    type JsonResponse = AccessPolicy;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        let base = if self.is_account {
            format!("accounts/{}", self.account_id)
        } else {
            format!("zones/{}", self.account_id)
        };

        match self.app_id {
            Some(app_id) => format!("{}/access/apps/{}/policies/{}", base, app_id, self.policy_id),
            None => format!("{}/access/policies/{}", base, self.policy_id),
        }
    }
}

/// Create an Access policy
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/policies/methods/create/
#[derive(Debug)]
pub struct CreateAccessPolicy<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Policy creation parameters
    pub params: CreateAccessPolicyParams,
    /// Application identifier (optional for account-level policies)
    pub app_id: Option<&'a str>,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for CreateAccessPolicy<'_> {
    type JsonResponse = AccessPolicy;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        let base = if self.is_account {
            format!("accounts/{}", self.account_id)
        } else {
            format!("zones/{}", self.account_id)
        };

        match self.app_id {
            Some(app_id) => format!("{}/access/apps/{}/policies", base, app_id),
            None => format!("{}/access/policies", base),
        }
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Update an Access policy
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/policies/methods/update/
#[derive(Debug)]
pub struct UpdateAccessPolicy<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Policy identifier
    pub policy_id: &'a str,
    /// Policy update parameters
    pub params: UpdateAccessPolicyParams,
    /// Application identifier (optional for account-level policies)
    pub app_id: Option<&'a str>,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for UpdateAccessPolicy<'_> {
    type JsonResponse = AccessPolicy;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        let base = if self.is_account {
            format!("accounts/{}", self.account_id)
        } else {
            format!("zones/{}", self.account_id)
        };

        match self.app_id {
            Some(app_id) => format!("{}/access/apps/{}/policies/{}", base, app_id, self.policy_id),
            None => format!("{}/access/policies/{}", base, self.policy_id),
        }
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Delete an Access policy
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/policies/methods/delete/
#[derive(Debug)]
pub struct DeleteAccessPolicy<'a> {
    /// Account or zone identifier
    pub account_id: &'a str,
    /// Policy identifier
    pub policy_id: &'a str,
    /// Application identifier (optional for account-level policies)
    pub app_id: Option<&'a str>,
    /// Is this an account-level request
    pub is_account: bool,
}

impl EndpointSpec for DeleteAccessPolicy<'_> {
    type JsonResponse = serde_json::Value; // Returns {"id": "policy_id"}
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        let base = if self.is_account {
            format!("accounts/{}", self.account_id)
        } else {
            format!("zones/{}", self.account_id)
        };

        match self.app_id {
            Some(app_id) => format!("{}/access/apps/{}/policies/{}", base, app_id, self.policy_id),
            None => format!("{}/access/policies/{}", base, self.policy_id),
        }
    }
}