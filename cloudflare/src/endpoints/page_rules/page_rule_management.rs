use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::page_rule_data_structures::{CreatePageRuleParams, PageRule, UpdatePageRuleParams};

/// List page rules for a zone
/// https://developers.cloudflare.com/api/resources/page_rules/methods/list/
#[derive(Debug)]
pub struct ListPageRules<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
    /// Status filter (active, disabled)
    pub status: Option<&'a str>,
    /// Order results (status, priority)
    pub order: Option<&'a str>,
    /// Direction to order results (asc, desc)  
    pub direction: Option<&'a str>,
    /// Match type for filtering (all, any)
    pub match_type: Option<&'a str>,
}

impl EndpointSpec for ListPageRules<'_> {
    type JsonResponse = Vec<PageRule>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules", self.zone_id)
    }

    fn query(&self) -> Option<String> {
        let mut params = Vec::new();

        if let Some(status) = self.status {
            params.push(format!("status={}", status));
        }

        if let Some(order) = self.order {
            params.push(format!("order={}", order));
        }

        if let Some(direction) = self.direction {
            params.push(format!("direction={}", direction));
        }

        if let Some(match_type) = self.match_type {
            params.push(format!("match={}", match_type));
        }

        if params.is_empty() {
            None
        } else {
            Some(params.join("&"))
        }
    }
}

/// Get a specific page rule
/// https://developers.cloudflare.com/api/resources/page_rules/methods/get/
#[derive(Debug)]
pub struct GetPageRule<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
    /// Page rule identifier
    pub rule_id: &'a str,
}

impl EndpointSpec for GetPageRule<'_> {
    type JsonResponse = PageRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules/{}", self.zone_id, self.rule_id)
    }
}

/// Create a new page rule
/// https://developers.cloudflare.com/api/resources/page_rules/methods/create/
#[derive(Debug)]
pub struct CreatePageRule<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
    /// Page rule creation parameters
    pub params: CreatePageRuleParams,
}

impl EndpointSpec for CreatePageRule<'_> {
    type JsonResponse = PageRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules", self.zone_id)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Update a page rule (full replacement)
/// https://developers.cloudflare.com/api/resources/page_rules/methods/update/
#[derive(Debug)]
pub struct UpdatePageRule<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
    /// Page rule identifier
    pub rule_id: &'a str,
    /// Page rule update parameters (full replacement)
    pub params: CreatePageRuleParams,
}

impl EndpointSpec for UpdatePageRule<'_> {
    type JsonResponse = PageRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules/{}", self.zone_id, self.rule_id)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Edit a page rule (partial update)
/// https://developers.cloudflare.com/api/resources/page_rules/methods/edit/
#[derive(Debug)]
pub struct EditPageRule<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
    /// Page rule identifier
    pub rule_id: &'a str,
    /// Page rule edit parameters (partial update)
    pub params: UpdatePageRuleParams,
}

impl EndpointSpec for EditPageRule<'_> {
    type JsonResponse = PageRule;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules/{}", self.zone_id, self.rule_id)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Delete a page rule
/// https://developers.cloudflare.com/api/resources/page_rules/methods/delete/
#[derive(Debug)]
pub struct DeletePageRule<'a> {
    /// Zone identifier
    pub zone_id: &'a str,
    /// Page rule identifier
    pub rule_id: &'a str,
}

impl EndpointSpec for DeletePageRule<'_> {
    type JsonResponse = serde_json::Value; // Returns {"id": "rule_id"}
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("zones/{}/pagerules/{}", self.zone_id, self.rule_id)
    }
}