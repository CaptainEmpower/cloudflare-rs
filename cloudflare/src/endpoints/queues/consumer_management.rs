use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::queue_data_structures::{Consumer, CreateConsumerParams};

/// Create or update a queue consumer
/// https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/create/
#[derive(Debug)]
pub struct CreateConsumer<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
    /// Consumer identifier
    pub consumer_id: &'a str,
    /// Consumer creation parameters
    pub params: CreateConsumerParams,
}

impl EndpointSpec for CreateConsumer<'_> {
    type JsonResponse = Consumer;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/queues/{}/consumers/{}",
            self.account_id, self.queue_id, self.consumer_id
        )
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Update an existing queue consumer
/// https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/update/
#[derive(Debug)]
pub struct UpdateConsumer<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
    /// Consumer identifier
    pub consumer_id: &'a str,
    /// Consumer update parameters
    pub params: CreateConsumerParams,
}

impl EndpointSpec for UpdateConsumer<'_> {
    type JsonResponse = Consumer;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/queues/{}/consumers/{}",
            self.account_id, self.queue_id, self.consumer_id
        )
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Get a specific queue consumer
/// https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/get/
#[derive(Debug)]
pub struct GetConsumer<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
    /// Consumer identifier
    pub consumer_id: &'a str,
}

impl EndpointSpec for GetConsumer<'_> {
    type JsonResponse = Consumer;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/queues/{}/consumers/{}",
            self.account_id, self.queue_id, self.consumer_id
        )
    }
}

/// Delete a queue consumer
/// https://developers.cloudflare.com/api/resources/queues/subresources/consumers/methods/delete/
#[derive(Debug)]
pub struct DeleteConsumer<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Queue identifier
    pub queue_id: &'a str,
    /// Consumer identifier
    pub consumer_id: &'a str,
}

impl EndpointSpec for DeleteConsumer<'_> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/queues/{}/consumers/{}",
            self.account_id, self.queue_id, self.consumer_id
        )
    }
}