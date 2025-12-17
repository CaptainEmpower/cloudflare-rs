use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::{ApiResponse, ApiResult},
};

/// List Durable Object namespaces owned by an account
#[derive(Debug)]
pub struct ListDurableObjectNamespaces<'a> {
    /// account ID where the namespaces are located
    pub account_id: &'a str,
    /// Number of namespaces per page (default 20, max 1000)
    pub per_page: Option<u32>,
    /// Page number for pagination (default 1)
    pub page: Option<u32>,
}

impl EndpointSpec for ListDurableObjectNamespaces<'_> {
    type JsonResponse = Vec<DurableObjectNamespace>;
    type ResponseType = ApiResponse<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/durable_objects/namespaces",
            self.account_id
        )
    }

    fn query(&self) -> Option<String> {
        let mut params = Vec::new();

        if let Some(per_page) = self.per_page {
            params.push(format!("per_page={}", per_page));
        }

        if let Some(page) = self.page {
            params.push(format!("page={}", page));
        }

        if params.is_empty() {
            None
        } else {
            Some(params.join("&"))
        }
    }
}

/// Represents a Durable Object namespace
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DurableObjectNamespace {
    /// Namespace identifier
    pub id: String,
    /// The class name associated with this namespace
    pub class: String,
    /// The name of the namespace
    pub name: String,
    /// Environment where this namespace exists
    pub environment: Option<String>,
    /// Script name associated with this namespace
    pub script: Option<String>,
}

impl ApiResult for DurableObjectNamespace {}
impl ApiResult for Vec<DurableObjectNamespace> {}
