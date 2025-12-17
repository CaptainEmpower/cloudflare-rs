use crate::framework::endpoint::{EndpointSpec, Method, MultipartBody, MultipartPart, RequestBody};
use crate::framework::response::ApiSuccess;

use super::script_data_structures::{WorkerScript, WorkerScriptContent, WorkerScriptMetadata};

/// Upload/Update a Workers script using legacy multipart API
/// https://developers.cloudflare.com/api/resources/workers/subresources/scripts/methods/update/
#[derive(Debug)]
pub struct UploadWorkerScript<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Script name
    pub script_name: &'a str,
    /// Script metadata
    pub metadata: WorkerScriptMetadata,
    /// Main script content
    pub script_content: WorkerScriptContent,
    /// Additional modules (WASM, data blobs, etc.)
    pub modules: Vec<WorkerScriptContent>,
}

impl EndpointSpec for UploadWorkerScript<'_> {
    type JsonResponse = WorkerScript;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}",
            self.account_id, self.script_name
        )
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::MultiPart(self))
    }
}

impl MultipartBody for UploadWorkerScript<'_> {
    fn parts(&self) -> Vec<(String, MultipartPart)> {
        let mut parts = Vec::new();

        // Add metadata part
        let metadata_json =
            serde_json::to_string(&self.metadata).unwrap_or_else(|_| "{}".to_string());
        parts.push(("metadata".to_string(), MultipartPart::Text(metadata_json)));

        // Add main script content
        parts.push((
            self.script_content.name.clone(),
            MultipartPart::Bytes(self.script_content.content.clone()),
        ));

        // Add additional modules
        for module in &self.modules {
            parts.push((
                module.name.clone(),
                MultipartPart::Bytes(module.content.clone()),
            ));
        }

        parts
    }
}

/// Upload a Workers script using the new 2025 JSON API (Beta)
/// https://developers.cloudflare.com/changelog/2025-09-03-new-workers-api/
#[derive(Debug)]
pub struct UploadWorkerScriptJson<'a> {
    /// Account identifier
    pub account_id: &'a str,
    /// Script name
    pub script_name: &'a str,
    /// Main module content (base64 encoded)
    pub main_module_content: String,
    /// Main module name
    pub main_module_name: String,
    /// Additional modules
    pub modules: Vec<(String, String, String)>, // (name, content_base64, content_type)
    /// Script metadata
    pub metadata: Option<WorkerScriptMetadata>,
}

impl EndpointSpec for UploadWorkerScriptJson<'_> {
    type JsonResponse = WorkerScript;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/workers/scripts/{}",
            self.account_id, self.script_name
        )
    }

    fn body(&self) -> Option<RequestBody> {
        use super::script_data_structures::{WorkerModuleUpload, WorkerScriptUploadRequest};

        let modules = if self.modules.is_empty() {
            None
        } else {
            Some(
                self.modules
                    .iter()
                    .map(|(name, content, content_type)| WorkerModuleUpload {
                        name: name.clone(),
                        content: content.clone(),
                        content_type: content_type.clone(),
                    })
                    .collect(),
            )
        };

        let request = WorkerScriptUploadRequest {
            main_module: self.main_module_content.clone(),
            modules,
            metadata: self.metadata.clone(),
        };

        Some(RequestBody::Json(serde_json::to_string(&request).unwrap()))
    }
}
