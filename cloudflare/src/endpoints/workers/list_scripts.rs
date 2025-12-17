use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

use super::script_data_structures::WorkerScript;

/// List Workers scripts
/// https://developers.cloudflare.com/api/resources/workers/subresources/scripts/methods/list/
#[derive(Debug)]
pub struct ListWorkerScripts<'a> {
    /// Account identifier
    pub account_id: &'a str,
}

impl EndpointSpec for ListWorkerScripts<'_> {
    type JsonResponse = Vec<WorkerScript>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/workers/scripts", self.account_id)
    }
}
