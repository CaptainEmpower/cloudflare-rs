#[cfg(test)]
mod tests {
    use super::super::{
        Consumer, ConsumerSettings, CreateConsumer, CreateConsumerParams, CreateQueue, CreateQueueParams,
        DeleteConsumer, DeleteQueue, GetConsumer, GetPurgeStatus, GetQueue, ListQueues,
        Producer, PurgeQueue, PurgeQueueParams, PurgeQueueResponse, Queue, QueueSettings,
        ScriptReference, UpdateConsumer, UpdateQueue, UpdateQueueParams,
    };
    use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
    use chrono::{DateTime, Utc};

    #[test]
    fn test_queue_serialization() {
        let queue = Queue {
            queue_id: "queue-123".to_string(),
            queue_name: "my-queue".to_string(),
            created_on: DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            modified_on: DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            producers: vec![Producer {
                script_ref: ScriptReference {
                    namespace: None,
                    script: Some("producer-worker".to_string()),
                    service: None,
                    environment: Some("production".to_string()),
                },
                producer_type: "worker".to_string(),
                bucket_name: None,
            }],
            producers_total_count: 1,
            consumers: vec![Consumer {
                script_ref: ScriptReference {
                    namespace: None,
                    script: Some("consumer-worker".to_string()),
                    service: None,
                    environment: Some("production".to_string()),
                },
                consumer_id: "consumer-456".to_string(),
                consumer_type: "worker".to_string(),
                settings: ConsumerSettings {
                    batch_size: Some(10),
                    max_retries: Some(3),
                    max_wait_time_ms: Some(5000),
                    max_concurrency: Some(5),
                    visibility_timeout_ms: Some(30000),
                    retry_delay: Some(10),
                },
                dead_letter_queue: None,
                bucket_name: None,
                created_on: None,
            }],
            consumers_total_count: 1,
            settings: Some(QueueSettings {
                delivery_delay: Some(30),
                delivery_paused: Some(false),
                message_retention_period: Some(86400),
            }),
        };

        let json = serde_json::to_string(&queue).unwrap();
        let deserialized: Queue = serde_json::from_str(&json).unwrap();

        assert_eq!(queue, deserialized);
        assert_eq!(queue.queue_id, "queue-123");
        assert_eq!(queue.queue_name, "my-queue");
        assert_eq!(queue.producers.len(), 1);
        assert_eq!(queue.consumers.len(), 1);
    }

    #[test]
    fn test_queue_settings_serialization() {
        let settings = QueueSettings {
            delivery_delay: Some(60),
            delivery_paused: Some(true),
            message_retention_period: Some(3600),
        };

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: QueueSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings, deserialized);
        assert_eq!(settings.delivery_delay, Some(60));
        assert_eq!(settings.delivery_paused, Some(true));
        assert_eq!(settings.message_retention_period, Some(3600));

        // Test optional fields
        let minimal_settings = QueueSettings {
            delivery_delay: None,
            delivery_paused: None,
            message_retention_period: None,
        };

        let minimal_json = serde_json::to_string(&minimal_settings).unwrap();
        let deserialized_minimal: QueueSettings = serde_json::from_str(&minimal_json).unwrap();

        assert_eq!(minimal_settings, deserialized_minimal);
    }

    #[test]
    fn test_consumer_settings_serialization() {
        let settings = ConsumerSettings {
            batch_size: Some(25),
            max_retries: Some(5),
            max_wait_time_ms: Some(10000),
            max_concurrency: Some(10),
            visibility_timeout_ms: Some(60000),
            retry_delay: Some(30),
        };

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: ConsumerSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings, deserialized);

        // Test all fields are properly mapped
        assert_eq!(settings.batch_size, Some(25));
        assert_eq!(settings.max_retries, Some(5));
        assert_eq!(settings.max_wait_time_ms, Some(10000));
        assert_eq!(settings.max_concurrency, Some(10));
        assert_eq!(settings.visibility_timeout_ms, Some(60000));
        assert_eq!(settings.retry_delay, Some(30));
    }

    #[test]
    fn test_script_reference_serialization() {
        let script_ref = ScriptReference {
            namespace: Some("my-namespace".to_string()),
            script: Some("my-script".to_string()),
            service: Some("my-service".to_string()),
            environment: Some("staging".to_string()),
        };

        let json = serde_json::to_string(&script_ref).unwrap();
        let deserialized: ScriptReference = serde_json::from_str(&json).unwrap();

        assert_eq!(script_ref, deserialized);

        // Test minimal script reference
        let minimal_ref = ScriptReference {
            namespace: None,
            script: None,
            service: None,
            environment: None,
        };

        let minimal_json = serde_json::to_string(&minimal_ref).unwrap();
        let deserialized_minimal: ScriptReference = serde_json::from_str(&minimal_json).unwrap();

        assert_eq!(minimal_ref, deserialized_minimal);
    }

    #[test]
    fn test_create_queue_endpoint() {
        let create_params = CreateQueueParams {
            queue_name: "test-queue".to_string(),
            settings: Some(QueueSettings {
                delivery_delay: Some(15),
                delivery_paused: Some(false),
                message_retention_period: Some(7200),
            }),
        };

        let create_request = CreateQueue {
            account_id: "test-account-123",
            params: create_params,
        };

        assert_eq!(create_request.method(), Method::POST);
        assert_eq!(create_request.path(), "accounts/test-account-123/queues");

        // Test body serialization
        let body = create_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"queue_name\":\"test-queue\""));
            assert!(json.contains("\"delivery_delay\":15"));
        }
    }

    #[test]
    fn test_list_queues_endpoint() {
        let list_request = ListQueues {
            account_id: "test-account-456",
            page: Some(2),
            name: Some("my-queue"),
        };

        assert_eq!(list_request.method(), Method::GET);
        assert_eq!(list_request.path(), "accounts/test-account-456/queues");

        // Test query parameters
        let query = list_request.query();
        assert!(query.is_some());
        let query_string = query.unwrap();
        assert!(query_string.contains("page=2"));
        assert!(query_string.contains("name=my-queue"));

        // Test without query parameters
        let list_request_no_params = ListQueues {
            account_id: "test-account",
            page: None,
            name: None,
        };

        assert_eq!(list_request_no_params.query(), None);
    }

    #[test]
    fn test_get_queue_endpoint() {
        let get_request = GetQueue {
            account_id: "test-account-789",
            queue_id: "queue-abc123",
        };

        assert_eq!(get_request.method(), Method::GET);
        assert_eq!(
            get_request.path(),
            "accounts/test-account-789/queues/queue-abc123"
        );
        assert_eq!(get_request.query(), None);
    }

    #[test]
    fn test_update_queue_endpoint() {
        let update_params = UpdateQueueParams {
            queue_name: "updated-queue".to_string(),
            settings: Some(QueueSettings {
                delivery_delay: Some(120),
                delivery_paused: Some(true),
                message_retention_period: Some(14400),
            }),
        };

        let update_request = UpdateQueue {
            account_id: "test-account-update",
            queue_id: "queue-update123",
            params: update_params,
        };

        assert_eq!(update_request.method(), Method::PATCH);
        assert_eq!(
            update_request.path(),
            "accounts/test-account-update/queues/queue-update123"
        );

        // Test body serialization
        let body = update_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"queue_name\":\"updated-queue\""));
            assert!(json.contains("\"delivery_paused\":true"));
        }
    }

    #[test]
    fn test_delete_queue_endpoint() {
        let delete_request = DeleteQueue {
            account_id: "test-account-delete",
            queue_id: "queue-delete123",
        };

        assert_eq!(delete_request.method(), Method::DELETE);
        assert_eq!(
            delete_request.path(),
            "accounts/test-account-delete/queues/queue-delete123"
        );
        assert_eq!(delete_request.query(), None);
    }

    #[test]
    fn test_create_consumer_endpoint() {
        let consumer_params = CreateConsumerParams {
            consumer_type: "worker".to_string(),
            script_name: Some("consumer-script".to_string()),
            environment_name: Some("production".to_string()),
            settings: ConsumerSettings {
                batch_size: Some(20),
                max_retries: Some(3),
                max_wait_time_ms: Some(8000),
                max_concurrency: Some(8),
                visibility_timeout_ms: Some(45000),
                retry_delay: Some(15),
            },
            dead_letter_queue: Some("dlq-queue".to_string()),
        };

        let create_consumer_request = CreateConsumer {
            account_id: "test-account-consumer",
            queue_id: "queue-consumer123",
            consumer_id: "consumer456",
            params: consumer_params,
        };

        assert_eq!(create_consumer_request.method(), Method::PUT);
        assert_eq!(
            create_consumer_request.path(),
            "accounts/test-account-consumer/queues/queue-consumer123/consumers/consumer456"
        );

        // Test body serialization
        let body = create_consumer_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"type\":\"worker\""));
            assert!(json.contains("\"script_name\":\"consumer-script\""));
            assert!(json.contains("\"batch_size\":20"));
        }
    }

    #[test]
    fn test_consumer_management_endpoints() {
        let consumer_params = CreateConsumerParams {
            consumer_type: "http_pull".to_string(),
            script_name: None,
            environment_name: None,
            settings: ConsumerSettings {
                batch_size: Some(5),
                max_retries: Some(1),
                max_wait_time_ms: Some(1000),
                max_concurrency: None,
                visibility_timeout_ms: Some(20000),
                retry_delay: Some(5),
            },
            dead_letter_queue: None,
        };

        // Test UpdateConsumer
        let update_consumer_request = UpdateConsumer {
            account_id: "test-account",
            queue_id: "queue123",
            consumer_id: "consumer789",
            params: consumer_params.clone(),
        };

        assert_eq!(update_consumer_request.method(), Method::POST);
        assert_eq!(
            update_consumer_request.path(),
            "accounts/test-account/queues/queue123/consumers/consumer789"
        );

        // Test GetConsumer
        let get_consumer_request = GetConsumer {
            account_id: "test-account",
            queue_id: "queue123",
            consumer_id: "consumer789",
        };

        assert_eq!(get_consumer_request.method(), Method::GET);
        assert_eq!(
            get_consumer_request.path(),
            "accounts/test-account/queues/queue123/consumers/consumer789"
        );

        // Test DeleteConsumer
        let delete_consumer_request = DeleteConsumer {
            account_id: "test-account",
            queue_id: "queue123",
            consumer_id: "consumer789",
        };

        assert_eq!(delete_consumer_request.method(), Method::DELETE);
        assert_eq!(
            delete_consumer_request.path(),
            "accounts/test-account/queues/queue123/consumers/consumer789"
        );
    }

    #[test]
    fn test_purge_queue_endpoint() {
        let purge_params = PurgeQueueParams {
            delete_messages_permanently: true,
        };

        let purge_request = PurgeQueue {
            account_id: "test-account-purge",
            queue_id: "queue-purge123",
            params: purge_params,
        };

        assert_eq!(purge_request.method(), Method::DELETE);
        assert_eq!(
            purge_request.path(),
            "accounts/test-account-purge/queues/queue-purge123/messages"
        );

        // Test body serialization
        let body = purge_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"delete_messages_permanently\":true"));
        }
    }

    #[test]
    fn test_get_purge_status_endpoint() {
        let purge_status_request = GetPurgeStatus {
            account_id: "test-account-status",
            queue_id: "queue-status123",
        };

        assert_eq!(purge_status_request.method(), Method::GET);
        assert_eq!(
            purge_status_request.path(),
            "accounts/test-account-status/queues/queue-status123/purge/status"
        );
        assert_eq!(purge_status_request.query(), None);
    }

    #[test]
    fn test_purge_queue_response_serialization() {
        let purge_response = PurgeQueueResponse {
            started_at: DateTime::parse_from_rfc3339("2023-12-01T10:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            complete: false,
        };

        let json = serde_json::to_string(&purge_response).unwrap();
        let deserialized: PurgeQueueResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(purge_response, deserialized);
        assert!(!deserialized.complete);

        // Test completed purge
        let completed_response = PurgeQueueResponse {
            started_at: DateTime::parse_from_rfc3339("2023-12-01T10:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            complete: true,
        };

        let completed_json = serde_json::to_string(&completed_response).unwrap();
        let deserialized_completed: PurgeQueueResponse =
            serde_json::from_str(&completed_json).unwrap();

        assert_eq!(completed_response, deserialized_completed);
        assert!(deserialized_completed.complete);
    }

    #[test]
    fn test_endpoint_paths_consistency() {
        // Verify all endpoint paths follow the expected pattern
        let account_id = "test-account";
        let queue_id = "queue-123";
        let consumer_id = "consumer-456";

        // Queue management paths
        let create_queue = CreateQueue {
            account_id,
            params: CreateQueueParams {
                queue_name: "test".to_string(),
                settings: None,
            },
        };
        assert_eq!(create_queue.path(), "accounts/test-account/queues");

        let get_queue = GetQueue {
            account_id,
            queue_id,
        };
        assert_eq!(get_queue.path(), "accounts/test-account/queues/queue-123");

        // Consumer management paths
        let get_consumer = GetConsumer {
            account_id,
            queue_id,
            consumer_id,
        };
        assert_eq!(
            get_consumer.path(),
            "accounts/test-account/queues/queue-123/consumers/consumer-456"
        );

        // Queue operations paths
        let purge_queue = PurgeQueue {
            account_id,
            queue_id,
            params: PurgeQueueParams {
                delete_messages_permanently: true,
            },
        };
        assert_eq!(
            purge_queue.path(),
            "accounts/test-account/queues/queue-123/messages"
        );

        let purge_status = GetPurgeStatus {
            account_id,
            queue_id,
        };
        assert_eq!(
            purge_status.path(),
            "accounts/test-account/queues/queue-123/purge/status"
        );
    }
}