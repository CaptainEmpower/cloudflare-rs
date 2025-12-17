use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::framework::response::ApiResult;

/// Queue configuration and metadata
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Queue {
    /// Queue identifier
    pub queue_id: String,
    /// Queue name
    pub queue_name: String,
    /// Creation timestamp
    pub created_on: DateTime<Utc>,
    /// Last modification timestamp
    pub modified_on: DateTime<Utc>,
    /// Queue producers
    pub producers: Vec<Producer>,
    /// Total count of producers
    pub producers_total_count: u32,
    /// Queue consumers
    pub consumers: Vec<Consumer>,
    /// Total count of consumers
    pub consumers_total_count: u32,
    /// Queue settings
    pub settings: Option<QueueSettings>,
}

/// Queue settings configuration
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct QueueSettings {
    /// Message delivery delay in seconds (0-42300)
    pub delivery_delay: Option<u32>,
    /// Whether message delivery is paused
    pub delivery_paused: Option<bool>,
    /// Message retention period in seconds (60-1209600)
    pub message_retention_period: Option<u32>,
}

/// Script reference for producers and consumers
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ScriptReference {
    /// Namespace for the script
    pub namespace: Option<String>,
    /// Script name
    pub script: Option<String>,
    /// Service name
    pub service: Option<String>,
    /// Environment name
    pub environment: Option<String>,
}

/// Queue producer configuration
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Producer {
    #[serde(flatten)]
    pub script_ref: ScriptReference,
    /// Producer type
    #[serde(rename = "type")]
    pub producer_type: String,
    /// Bucket name (for R2 event notifications)
    pub bucket_name: Option<String>,
}

/// Queue consumer configuration
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Consumer {
    #[serde(flatten)]
    pub script_ref: ScriptReference,
    /// Consumer identifier
    pub consumer_id: String,
    /// Consumer type
    #[serde(rename = "type")]
    pub consumer_type: String,
    /// Consumer settings
    pub settings: ConsumerSettings,
    /// Dead letter queue name
    pub dead_letter_queue: Option<String>,
    /// Bucket name (for R2 event notifications)
    pub bucket_name: Option<String>,
    /// Creation timestamp (when returned from API)
    pub created_on: Option<DateTime<Utc>>,
}

/// Consumer settings configuration
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ConsumerSettings {
    /// Number of messages in each batch (1-100)
    pub batch_size: Option<u32>,
    /// Maximum number of retries (0-100)
    pub max_retries: Option<u32>,
    /// Maximum wait time for batch in milliseconds (0-30000)
    pub max_wait_time_ms: Option<u32>,
    /// Maximum concurrent executions (null for unlimited)
    pub max_concurrency: Option<u32>,
    /// Message visibility timeout in milliseconds (0-43200000)
    pub visibility_timeout_ms: Option<u32>,
    /// Retry delay in seconds (1-86400)
    pub retry_delay: Option<u32>,
}

/// Parameters for creating a new queue
#[derive(Serialize, Debug, Clone)]
pub struct CreateQueueParams {
    /// Queue name (1-63 characters, alphanumeric and dashes only)
    pub queue_name: String,
    /// Optional queue settings
    pub settings: Option<QueueSettings>,
}

/// Parameters for updating an existing queue
#[derive(Serialize, Debug, Clone)]
pub struct UpdateQueueParams {
    /// Queue name
    pub queue_name: String,
    /// Queue settings to update
    pub settings: Option<QueueSettings>,
}

/// Parameters for creating/updating a consumer
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct CreateConsumerParams {
    /// Consumer type ("worker" or "http_pull")
    #[serde(rename = "type")]
    pub consumer_type: String,
    /// Script name for worker consumers
    pub script_name: Option<String>,
    /// Environment name for worker consumers
    pub environment_name: Option<String>,
    /// Consumer settings
    pub settings: ConsumerSettings,
    /// Dead letter queue name
    pub dead_letter_queue: Option<String>,
}

/// Parameters for purging queue messages
#[derive(Serialize, Debug, Clone)]
pub struct PurgeQueueParams {
    /// Whether to permanently delete messages (required: true)
    pub delete_messages_permanently: bool,
}

/// Response from queue purge operation
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PurgeQueueResponse {
    /// When the purge operation started
    pub started_at: DateTime<Utc>,
    /// Whether the purge is complete
    pub complete: bool,
}

/// Response for listing queues
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct QueueListResponse {
    /// List of queues
    pub result: Vec<Queue>,
}

// Implement ApiResult for all response types
impl ApiResult for Queue {}
impl ApiResult for Vec<Queue> {}
impl ApiResult for PurgeQueueResponse {}
impl ApiResult for QueueListResponse {}
impl ApiResult for Consumer {}