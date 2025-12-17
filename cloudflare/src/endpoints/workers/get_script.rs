use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

use super::script_data_structures::WorkerScript;

/// Get a Workers script by name
/// https://developers.cloudflare.com/api/resources/workers/subresources/scripts/methods/get/
#[derive(Debug)]
pub struct GetWorkerScript<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Script name
    pub script_name: &'a str,
}

impl EndpointSpec for GetWorkerScript<'_> {
    type JsonResponse = WorkerScript;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}",
            self.account_id, self.script_name
        )
    }
}
