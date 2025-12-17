use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::queue_data_structures::{CreateQueueParams, Queue, UpdateQueueParams};

/// Create a new queue
/// https://developers.cloudflare.com/api/resources/queues/methods/create/
#[derive(Debug)]
pub struct CreateQueue<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue creation parameters
    pub params: CreateQueueParams,
}

impl EndpointSpec for CreateQueue<'_> {
    type JsonResponse = Queue;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("accounts/{}/queues", self.account_id)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// List queues in account
/// https://developers.cloudflare.com/api/resources/queues/methods/list/
#[derive(Debug)]
pub struct ListQueues<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Page number for pagination (optional)
    pub page: Option<u32>,
    /// Filter by queue name (optional)
    pub name: Option<&'a str>,
}

impl EndpointSpec for ListQueues<'_> {
    type JsonResponse = Vec<Queue>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/queues", self.account_id)
    }

    fn query(&self) -> Option<String> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(format!("page={}", page));
        }

        if let Some(name) = self.name {
            params.push(format!("name={}", name));
        }

        if params.is_empty() {
            None
        } else {
            Some(params.join("&"))
        }
    }
}

/// Get a specific queue by ID
/// https://developers.cloudflare.com/api/resources/queues/methods/get/
#[derive(Debug)]
pub struct GetQueue<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
}

impl EndpointSpec for GetQueue<'_> {
    type JsonResponse = Queue;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/queues/{}", self.account_id, self.queue_id)
    }
}

/// Update queue configuration
/// https://developers.cloudflare.com/api/resources/queues/methods/update/
#[derive(Debug)]
pub struct UpdateQueue<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
    /// Queue update parameters
    pub params: UpdateQueueParams,
}

impl EndpointSpec for UpdateQueue<'_> {
    type JsonResponse = Queue;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!("accounts/{}/queues/{}", self.account_id, self.queue_id)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Delete a queue
/// https://developers.cloudflare.com/api/resources/queues/methods/delete/
#[derive(Debug)]
pub struct DeleteQueue<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
}

impl EndpointSpec for DeleteQueue<'_> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("accounts/{}/queues/{}", self.account_id, self.queue_id)
    }
}