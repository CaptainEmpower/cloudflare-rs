use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::queue_data_structures::{PurgeQueueParams, PurgeQueueResponse};

/// Purge all messages from a queue
/// https://developers.cloudflare.com/api/resources/queues/methods/purge/
#[derive(Debug)]
pub struct PurgeQueue<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
    /// Purge parameters
    pub params: PurgeQueueParams,
}

impl EndpointSpec for PurgeQueue<'_> {
    type JsonResponse = PurgeQueueResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/queues/{}/messages",
            self.account_id, self.queue_id
        )
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Get the status of a queue purge operation
/// https://developers.cloudflare.com/api/resources/queues/methods/purge-status/
#[derive(Debug)]
pub struct GetPurgeStatus<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
}

impl EndpointSpec for GetPurgeStatus<'_> {
    type JsonResponse = PurgeQueueResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/queues/{}/purge/status",
            self.account_id, self.queue_id
        )
    }
}