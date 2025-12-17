use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::framework::response::ApiResult;

/// A Workers script representation
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkerScript {
    /// Script identifier
    pub id: String,
    /// Script name
    pub name: Option<String>,
    /// Script creation date
    pub created_on: Option<DateTime<Utc>>,
    /// Script last modified date
    pub modified_on: Option<DateTime<Utc>>,
    /// Compatibility date for the script
    pub compatibility_date: Option<String>,
    /// Compatibility flags
    pub compatibility_flags: Option<Vec<String>>,
    /// Event handlers supported by the script
    pub handlers: Option<Vec<String>>,
    /// Script tags
    pub tags: Option<Vec<String>>,
    /// Usage model
    pub usage_model: Option<WorkerUsageModel>,
    /// Whether the script is deployed
    pub deployed: Option<bool>,
    /// Deployment ID
    pub deployment_id: Option<String>,
    /// Script placement
    pub placement: Option<WorkerPlacement>,
    /// Logpush configuration
    pub logpush: Option<bool>,
    /// Tail consumer settings
    pub tail_consumers: Option<Vec<WorkerTailConsumer>>,
}

/// Workers usage model
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkerUsageModel {
    /// Standard usage model
    Standard,
    /// Unbound usage model
    Unbound,
}

/// Worker placement configuration
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkerPlacement {
    /// Placement mode
    pub mode: Option<String>,
}

/// Tail consumer configuration
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkerTailConsumer {
    /// Consumer service name
    pub service: String,
    /// Consumer environment
    pub environment: Option<String>,
    /// Consumer namespace
    pub namespace: Option<String>,
}

/// Worker script metadata for uploads
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct WorkerScriptMetadata {
    /// Main module name (entry point)
    pub main_module: String,
    /// Worker bindings
    pub bindings: Option<Vec<WorkerBinding>>,
    /// Compatibility date
    pub compatibility_date: Option<String>,
    /// Compatibility flags
    pub compatibility_flags: Option<Vec<String>>,
    /// Usage model
    pub usage_model: Option<WorkerUsageModel>,
    /// Script placement
    pub placement: Option<WorkerPlacement>,
    /// Tail consumers
    pub tail_consumers: Option<Vec<WorkerTailConsumer>>,
    /// Keep bindings on error
    pub keep_bindings_on_error: Option<bool>,
    /// Observability configuration
    pub observability: Option<WorkerObservability>,
}

/// Worker observability settings
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkerObservability {
    /// Logpush enabled
    pub logpush: Option<bool>,
}

/// Worker binding types for configuration
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkerBinding {
    /// Plain text binding
    PlainText { name: String, text: String },
    /// JSON binding  
    Json {
        name: String,
        json: serde_json::Value,
    },
    /// Environment variable binding
    Environment { name: String, text: String },
    /// Secret text binding
    SecretText { name: String, text: String },
    /// WebAssembly module binding
    WasmModule { name: String, part: String },
    /// Data blob binding
    DataBlob { name: String, part: String },
    /// KV namespace binding
    KvNamespace { name: String, namespace_id: String },
    /// Durable Object namespace binding
    DurableObjectNamespace {
        name: String,
        class_name: String,
        script_name: Option<String>,
        environment: Option<String>,
    },
    /// R2 bucket binding
    R2Bucket {
        name: String,
        bucket_name: String,
        jurisdiction: Option<String>,
    },
    /// Queue binding
    Queue { name: String, queue_name: String },
    /// D1 database binding
    D1Database { name: String, database_id: String },
    /// Service binding
    Service {
        name: String,
        service: String,
        environment: Option<String>,
    },
    /// Analytics Engine binding
    AnalyticsEngine { name: String, dataset: String },
    /// mTLS certificate binding
    MtlsCertificate {
        name: String,
        certificate_id: String,
    },
    /// Logfwdr binding
    Logfwdr { name: String, destination: String },
    /// AI binding
    Ai { name: String },
    /// Version metadata binding
    VersionMetadata { name: String },
}

/// Worker script content for multipart uploads
#[derive(Debug, Clone)]
pub struct WorkerScriptContent {
    /// Script content as bytes
    pub content: Vec<u8>,
    /// Content type (e.g., "application/javascript+module")
    pub content_type: String,
    /// Module name
    pub name: String,
}

/// Worker module for script uploads
#[derive(Debug, Clone)]
pub struct WorkerModule {
    /// Module name
    pub name: String,
    /// Module content
    pub content: Vec<u8>,
    /// Content type
    pub content_type: String,
}

/// List scripts response
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ListWorkersResponse {
    /// Array of worker scripts
    pub result: Vec<WorkerScript>,
}

/// Worker script upload response (2025 API)
#[skip_serializing_none]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkerScriptResponse {
    /// Script details
    pub script: Option<WorkerScript>,
    /// Success flag
    pub success: Option<bool>,
    /// Error messages
    pub errors: Option<Vec<String>>,
    /// Warning messages  
    pub messages: Option<Vec<String>>,
}

/// Worker script upload request for 2025 JSON API
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct WorkerScriptUploadRequest {
    /// Main module content (base64 encoded)
    pub main_module: String,
    /// Additional modules
    pub modules: Option<Vec<WorkerModuleUpload>>,
    /// Worker metadata
    pub metadata: Option<WorkerScriptMetadata>,
}

/// Module upload for 2025 API
#[derive(Serialize, Debug, Clone)]
pub struct WorkerModuleUpload {
    /// Module name
    pub name: String,
    /// Module content (base64 encoded)  
    pub content: String,
    /// Content type
    #[serde(rename = "type")]
    pub content_type: String,
}

// Implement ApiResult for all response types
impl ApiResult for WorkerScript {}
impl ApiResult for ListWorkersResponse {}
impl ApiResult for WorkerScriptResponse {}
impl ApiResult for Vec<WorkerScript> {}
