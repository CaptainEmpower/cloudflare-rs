#[cfg(test)]
mod tests {
    use super::super::{
        script_data_structures::{
            WorkerBinding, WorkerObservability, WorkerPlacement, WorkerScript, WorkerScriptContent,
            WorkerScriptMetadata, WorkerTailConsumer, WorkerUsageModel,
        },
        GetWorkerScript, ListWorkerScripts, UploadWorkerScript, UploadWorkerScriptJson,
    };
    use crate::framework::endpoint::{EndpointSpec, Method, MultipartBody, RequestBody};
    use chrono::{DateTime, Utc};

    #[test]
    fn test_worker_script_serialization() {
        let worker = WorkerScript {
            id: "test-worker-123".to_string(),
            name: Some("my-worker".to_string()),
            created_on: Some(
                DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            modified_on: Some(
                DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            compatibility_date: Some("2023-01-01".to_string()),
            compatibility_flags: Some(vec!["new_feature".to_string()]),
            handlers: Some(vec!["fetch".to_string(), "scheduled".to_string()]),
            tags: Some(vec!["production".to_string()]),
            usage_model: Some(WorkerUsageModel::Standard),
            deployed: Some(true),
            deployment_id: Some("deploy-123".to_string()),
            placement: Some(WorkerPlacement {
                mode: Some("smart".to_string()),
            }),
            logpush: Some(true),
            tail_consumers: Some(vec![WorkerTailConsumer {
                service: "log-service".to_string(),
                environment: Some("prod".to_string()),
                namespace: Some("default".to_string()),
            }]),
        };

        let json = serde_json::to_string(&worker).unwrap();
        let deserialized: WorkerScript = serde_json::from_str(&json).unwrap();

        assert_eq!(worker, deserialized);
        assert_eq!(worker.id, "test-worker-123");
        assert_eq!(worker.name, Some("my-worker".to_string()));
        assert_eq!(worker.usage_model, Some(WorkerUsageModel::Standard));
    }

    #[test]
    fn test_worker_script_minimal() {
        let worker = WorkerScript {
            id: "minimal-worker".to_string(),
            name: None,
            created_on: None,
            modified_on: None,
            compatibility_date: None,
            compatibility_flags: None,
            handlers: None,
            tags: None,
            usage_model: None,
            deployed: None,
            deployment_id: None,
            placement: None,
            logpush: None,
            tail_consumers: None,
        };

        let json = serde_json::to_string(&worker).unwrap();
        let deserialized: WorkerScript = serde_json::from_str(&json).unwrap();

        assert_eq!(worker, deserialized);
        assert_eq!(worker.id, "minimal-worker");
        assert!(worker.name.is_none());
        assert!(worker.usage_model.is_none());
    }

    #[test]
    fn test_worker_usage_model_serialization() {
        assert_eq!(
            serde_json::to_string(&WorkerUsageModel::Standard).unwrap(),
            "\"standard\""
        );
        assert_eq!(
            serde_json::to_string(&WorkerUsageModel::Unbound).unwrap(),
            "\"unbound\""
        );

        assert_eq!(
            serde_json::from_str::<WorkerUsageModel>("\"standard\"").unwrap(),
            WorkerUsageModel::Standard
        );
        assert_eq!(
            serde_json::from_str::<WorkerUsageModel>("\"unbound\"").unwrap(),
            WorkerUsageModel::Unbound
        );
    }

    #[test]
    fn test_worker_bindings_serialization() {
        let bindings = vec![
            WorkerBinding::PlainText {
                name: "MESSAGE".to_string(),
                text: "Hello World".to_string(),
            },
            WorkerBinding::Json {
                name: "CONFIG".to_string(),
                json: serde_json::json!({"key": "value"}),
            },
            WorkerBinding::KvNamespace {
                name: "MY_KV".to_string(),
                namespace_id: "kv-123".to_string(),
            },
            WorkerBinding::R2Bucket {
                name: "MY_BUCKET".to_string(),
                bucket_name: "storage".to_string(),
                jurisdiction: Some("eu".to_string()),
            },
            WorkerBinding::D1Database {
                name: "MY_DB".to_string(),
                database_id: "db-456".to_string(),
            },
        ];

        let json = serde_json::to_string(&bindings).unwrap();
        let deserialized: Vec<WorkerBinding> = serde_json::from_str(&json).unwrap();

        assert_eq!(bindings, deserialized);
        assert_eq!(bindings.len(), 5);

        // Verify specific binding types
        match &bindings[0] {
            WorkerBinding::PlainText { name, text } => {
                assert_eq!(name, "MESSAGE");
                assert_eq!(text, "Hello World");
            }
            _ => panic!("Expected PlainText binding"),
        }

        match &bindings[1] {
            WorkerBinding::Json { name, json } => {
                assert_eq!(name, "CONFIG");
                assert_eq!(json["key"], "value");
            }
            _ => panic!("Expected Json binding"),
        }
    }

    #[test]
    fn test_worker_script_metadata() {
        let metadata = WorkerScriptMetadata {
            main_module: "worker.mjs".to_string(),
            bindings: Some(vec![WorkerBinding::PlainText {
                name: "API_KEY".to_string(),
                text: "secret123".to_string(),
            }]),
            compatibility_date: Some("2023-01-01".to_string()),
            compatibility_flags: Some(vec!["new_feature".to_string()]),
            usage_model: Some(WorkerUsageModel::Unbound),
            placement: Some(WorkerPlacement {
                mode: Some("smart".to_string()),
            }),
            tail_consumers: None,
            keep_bindings_on_error: Some(true),
            observability: Some(WorkerObservability {
                logpush: Some(true),
            }),
        };

        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains("\"main_module\":\"worker.mjs\""));
        assert!(json.contains("\"compatibility_date\":\"2023-01-01\""));
        assert!(json.contains("\"usage_model\":\"unbound\""));
    }

    #[test]
    fn test_list_worker_scripts_endpoint() {
        let list_request = ListWorkerScripts {
            account_id: "test-account-123",
        };

        assert_eq!(list_request.method(), Method::GET);
        assert_eq!(
            list_request.path(),
            "accounts/test-account-123/workers/scripts"
        );
        assert_eq!(list_request.query(), None);
    }

    #[test]
    fn test_get_worker_script_endpoint() {
        let get_request = GetWorkerScript {
            account_id: "test-account-456",
            script_name: "my-worker",
        };

        assert_eq!(get_request.method(), Method::GET);
        assert_eq!(
            get_request.path(),
            "accounts/test-account-456/workers/scripts/my-worker"
        );
        assert_eq!(get_request.query(), None);
    }

    #[test]
    fn test_upload_worker_script_endpoint() {
        let metadata = WorkerScriptMetadata {
            main_module: "worker.js".to_string(),
            bindings: None,
            compatibility_date: Some("2023-01-01".to_string()),
            compatibility_flags: None,
            usage_model: Some(WorkerUsageModel::Standard),
            placement: None,
            tail_consumers: None,
            keep_bindings_on_error: None,
            observability: None,
        };

        let script_content = WorkerScriptContent {
            content: b"export default { fetch() { return new Response('Hello'); } }".to_vec(),
            content_type: "application/javascript+module".to_string(),
            name: "worker.js".to_string(),
        };

        let upload_request = UploadWorkerScript {
            account_id: "test-account-789",
            script_name: "test-worker",
            metadata,
            script_content,
            modules: vec![],
        };

        assert_eq!(upload_request.method(), Method::PUT);
        assert_eq!(
            upload_request.path(),
            "accounts/test-account-789/workers/scripts/test-worker"
        );

        // Test multipart body generation
        let parts = upload_request.parts();
        assert_eq!(parts.len(), 2); // metadata + script content
        assert_eq!(parts[0].0, "metadata");
        assert_eq!(parts[1].0, "worker.js");
    }

    #[test]
    fn test_upload_worker_script_with_modules() {
        let metadata = WorkerScriptMetadata {
            main_module: "index.js".to_string(),
            bindings: Some(vec![WorkerBinding::WasmModule {
                name: "WASM".to_string(),
                part: "wasm_module".to_string(),
            }]),
            compatibility_date: Some("2023-01-01".to_string()),
            compatibility_flags: None,
            usage_model: None,
            placement: None,
            tail_consumers: None,
            keep_bindings_on_error: None,
            observability: None,
        };

        let script_content = WorkerScriptContent {
            content: b"import('./wasm_module').then(...)".to_vec(),
            content_type: "application/javascript+module".to_string(),
            name: "index.js".to_string(),
        };

        let wasm_module = WorkerScriptContent {
            content: vec![0, 97, 115, 109], // WebAssembly magic bytes
            content_type: "application/wasm".to_string(),
            name: "wasm_module".to_string(),
        };

        let upload_request = UploadWorkerScript {
            account_id: "test-account",
            script_name: "complex-worker",
            metadata,
            script_content,
            modules: vec![wasm_module],
        };

        // Test multipart body generation with modules
        let parts = upload_request.parts();
        assert_eq!(parts.len(), 3); // metadata + script + wasm module
        assert_eq!(parts[0].0, "metadata");
        assert_eq!(parts[1].0, "index.js");
        assert_eq!(parts[2].0, "wasm_module");
    }

    #[test]
    fn test_upload_worker_script_json_endpoint() {
        let metadata = WorkerScriptMetadata {
            main_module: "worker.mjs".to_string(),
            bindings: None,
            compatibility_date: Some("2023-01-01".to_string()),
            compatibility_flags: None,
            usage_model: None,
            placement: None,
            tail_consumers: None,
            keep_bindings_on_error: None,
            observability: None,
        };

        let upload_request = UploadWorkerScriptJson {
            account_id: "test-account-new",
            script_name: "json-worker",
            main_module_content: "ZXhwb3J0IGRlZmF1bHQgeyBmZXRjaCgpIHt9IH0=".to_string(), // base64 encoded
            main_module_name: "worker.mjs".to_string(),
            modules: vec![],
            metadata: Some(metadata),
        };

        assert_eq!(upload_request.method(), Method::PUT);
        assert_eq!(
            upload_request.path(),
            "accounts/test-account-new/workers/scripts/json-worker"
        );

        // Test JSON body generation
        let body = upload_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"main_module\":"));
            assert!(json.contains("\"metadata\":"));
        }
    }

    #[test]
    fn test_worker_binding_types_completeness() {
        // Test all major binding types for completeness
        let bindings = vec![
            WorkerBinding::PlainText {
                name: "TEXT".to_string(),
                text: "value".to_string(),
            },
            WorkerBinding::Json {
                name: "JSON".to_string(),
                json: serde_json::json!({}),
            },
            WorkerBinding::Environment {
                name: "ENV".to_string(),
                text: "prod".to_string(),
            },
            WorkerBinding::SecretText {
                name: "SECRET".to_string(),
                text: "hidden".to_string(),
            },
            WorkerBinding::WasmModule {
                name: "WASM".to_string(),
                part: "module".to_string(),
            },
            WorkerBinding::DataBlob {
                name: "DATA".to_string(),
                part: "blob".to_string(),
            },
            WorkerBinding::KvNamespace {
                name: "KV".to_string(),
                namespace_id: "kv123".to_string(),
            },
            WorkerBinding::DurableObjectNamespace {
                name: "DO".to_string(),
                class_name: "Counter".to_string(),
                script_name: None,
                environment: None,
            },
            WorkerBinding::R2Bucket {
                name: "BUCKET".to_string(),
                bucket_name: "storage".to_string(),
                jurisdiction: None,
            },
            WorkerBinding::Queue {
                name: "QUEUE".to_string(),
                queue_name: "tasks".to_string(),
            },
            WorkerBinding::D1Database {
                name: "DB".to_string(),
                database_id: "db123".to_string(),
            },
            WorkerBinding::Service {
                name: "SERVICE".to_string(),
                service: "backend".to_string(),
                environment: None,
            },
            WorkerBinding::AnalyticsEngine {
                name: "ANALYTICS".to_string(),
                dataset: "events".to_string(),
            },
            WorkerBinding::MtlsCertificate {
                name: "CERT".to_string(),
                certificate_id: "cert123".to_string(),
            },
            WorkerBinding::Logfwdr {
                name: "LOG".to_string(),
                destination: "splunk".to_string(),
            },
            WorkerBinding::Ai {
                name: "AI".to_string(),
            },
            WorkerBinding::VersionMetadata {
                name: "VERSION".to_string(),
            },
        ];

        // Test that all bindings serialize/deserialize correctly
        let json = serde_json::to_string(&bindings).unwrap();
        let deserialized: Vec<WorkerBinding> = serde_json::from_str(&json).unwrap();

        assert_eq!(bindings, deserialized);
        assert_eq!(bindings.len(), 17); // All binding types covered
    }
}
