use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

use super::access_data_structures::AccessUser;

/// List Access users
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/users/methods/list/
#[derive(Debug)]
pub struct ListAccessUsers<'a> {
    /// Account identifier
    pub account_id: &'a str,
}

impl EndpointSpec for ListAccessUsers<'_> {
    type JsonResponse = Vec<AccessUser>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/access/users", self.account_id)
    }
}

/// Get an Access user
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/users/methods/get/
#[derive(Debug)]
pub struct GetAccessUser<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// User identifier
    pub user_id: &'a str,
}

impl EndpointSpec for GetAccessUser<'_> {
    type JsonResponse = AccessUser;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/access/users/{}", self.account_id, self.user_id)
    }
}

/// Get failed Access logins
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/users/methods/list_failed_logins/
#[derive(Debug)]
pub struct GetFailedAccessLogins<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// User identifier
    pub user_id: &'a str,
}

impl EndpointSpec for GetFailedAccessLogins<'_> {
    type JsonResponse = serde_json::Value; // Contains failed login details
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/access/users/{}/failed_logins",
            self.account_id, self.user_id
        )
    }
}

/// Get Access user's active sessions
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/users/methods/list_active_sessions/
#[derive(Debug)]
pub struct GetAccessUserActiveSessions<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// User identifier
    pub user_id: &'a str,
}

impl EndpointSpec for GetAccessUserActiveSessions<'_> {
    type JsonResponse = serde_json::Value; // Contains active session details
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/access/users/{}/active_sessions",
            self.account_id, self.user_id
        )
    }
}

/// Revoke Access user sessions
/// https://developers.cloudflare.com/api/resources/zero_trust/subresources/access/subresources/users/methods/revoke_user_sessions/
#[derive(Debug)]
pub struct RevokeAccessUserSessions<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// User identifier
    pub user_id: &'a str,
}

impl EndpointSpec for RevokeAccessUserSessions<'_> {
    type JsonResponse = serde_json::Value; // Returns success confirmation
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/access/users/{}/revoke_sessions",
            self.account_id, self.user_id
        )
    }
}