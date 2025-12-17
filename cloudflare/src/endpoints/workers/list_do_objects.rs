use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::{ApiResponse, ApiResult},
};

/// List Durable Objects in a given namespace
#[derive(Debug)]
pub struct ListDurableObjectsInNamespace<'a> {
    /// account ID where the namespace is located
    pub account_id: &'a str,
    /// namespace ID to list objects from
    pub namespace_id: &'a str,
    /// Cursor for pagination (optional)
    pub cursor: Option<&'a str>,
    /// Number of objects to return (default 1000, max 10000)
    pub limit: Option<u32>,
}

impl EndpointSpec for ListDurableObjectsInNamespace<'_> {
    type JsonResponse = ListDurableObjectsResponse;
    type ResponseType = ApiResponse<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/durable_objects/namespaces/{}/objects",
            self.account_id, self.namespace_id
        )
    }

    fn query(&self) -> Option<String> {
        let mut params = Vec::new();

        if let Some(cursor) = self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }

        if params.is_empty() {
            None
        } else {
            Some(params.join("&"))
        }
    }
}

/// Response for listing Durable Objects in a namespace
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ListDurableObjectsResponse {
    /// Array of Durable Objects in the namespace
    pub result: Vec<DurableObjectInfo>,
    /// Cursor for next page of results (if available)
    pub result_info: Option<DurableObjectsResultInfo>,
}

/// Information about pagination for Durable Objects listing
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DurableObjectsResultInfo {
    /// Cursor for the next page of results
    pub cursor: Option<String>,
    /// Total count of objects (if available)
    pub count: Option<u32>,
}

/// Represents a Durable Object instance in a namespace
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DurableObjectInfo {
    /// Object identifier
    pub id: String,
    /// Whether this object has stored data
    #[serde(rename = "hasStoredData")]
    pub has_stored_data: bool,
}

impl ApiResult for ListDurableObjectsResponse {}
impl ApiResult for DurableObjectInfo {}
impl ApiResult for Vec<DurableObjectInfo> {}
