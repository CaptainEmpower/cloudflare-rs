#[cfg(test)]
mod tests {
    use super::super::{
        DurableObjectInfo, DurableObjectNamespace, DurableObjectsResultInfo,
        ListDurableObjectNamespaces, ListDurableObjectsInNamespace, ListDurableObjectsResponse,
    };
    use crate::framework::endpoint::EndpointSpec;

    #[test]
    fn test_list_do_namespaces_serialization() {
        let namespace = DurableObjectNamespace {
            id: "test-namespace-id".to_string(),
            class: "TestClass".to_string(),
            name: "test-namespace".to_string(),
            environment: Some("production".to_string()),
            script: Some("test-script".to_string()),
        };

        let json = serde_json::to_string(&namespace).unwrap();
        let deserialized: DurableObjectNamespace = serde_json::from_str(&json).unwrap();

        assert_eq!(namespace, deserialized);
        assert_eq!(namespace.id, "test-namespace-id");
        assert_eq!(namespace.class, "TestClass");
        assert_eq!(namespace.name, "test-namespace");
        assert_eq!(namespace.environment, Some("production".to_string()));
        assert_eq!(namespace.script, Some("test-script".to_string()));
    }

    #[test]
    fn test_list_do_namespaces_optional_fields() {
        let namespace = DurableObjectNamespace {
            id: "minimal-namespace".to_string(),
            class: "MinimalClass".to_string(),
            name: "minimal".to_string(),
            environment: None,
            script: None,
        };

        let json = serde_json::to_string(&namespace).unwrap();
        let deserialized: DurableObjectNamespace = serde_json::from_str(&json).unwrap();

        assert_eq!(namespace, deserialized);
        assert_eq!(namespace.environment, None);
        assert_eq!(namespace.script, None);
    }

    #[test]
    fn test_list_do_objects_response_serialization() {
        let object_info = DurableObjectInfo {
            id: "object-123".to_string(),
            has_stored_data: true,
        };

        let response = ListDurableObjectsResponse {
            result: vec![object_info.clone()],
            result_info: Some(DurableObjectsResultInfo {
                cursor: Some("next-cursor-token".to_string()),
                count: Some(1),
            }),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: ListDurableObjectsResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, deserialized);
        assert_eq!(response.result.len(), 1);
        assert_eq!(response.result[0].id, "object-123");
        assert_eq!(response.result[0].has_stored_data, true);
        assert!(response.result_info.is_some());
        assert_eq!(
            response.result_info.as_ref().unwrap().cursor,
            Some("next-cursor-token".to_string())
        );
    }

    #[test]
    fn test_durable_object_info_serialization() {
        let object_with_data = DurableObjectInfo {
            id: "obj-with-data".to_string(),
            has_stored_data: true,
        };

        let object_without_data = DurableObjectInfo {
            id: "obj-without-data".to_string(),
            has_stored_data: false,
        };

        // Test object with data
        let json1 = serde_json::to_string(&object_with_data).unwrap();
        let deserialized1: DurableObjectInfo = serde_json::from_str(&json1).unwrap();
        assert_eq!(object_with_data, deserialized1);
        assert!(deserialized1.has_stored_data);

        // Test object without data
        let json2 = serde_json::to_string(&object_without_data).unwrap();
        let deserialized2: DurableObjectInfo = serde_json::from_str(&json2).unwrap();
        assert_eq!(object_without_data, deserialized2);
        assert!(!deserialized2.has_stored_data);
    }

    #[test]
    fn test_list_do_namespaces_endpoint_path() {
        let list_request = ListDurableObjectNamespaces {
            account_id: "test-account-123",
            per_page: None,
            page: None,
        };

        assert_eq!(
            list_request.path(),
            "accounts/test-account-123/workers/durable_objects/namespaces"
        );
    }

    #[test]
    fn test_list_do_namespaces_query_params() {
        // Test with no parameters
        let request_no_params = ListDurableObjectNamespaces {
            account_id: "test-account",
            per_page: None,
            page: None,
        };
        assert_eq!(request_no_params.query(), None);

        // Test with per_page only
        let request_per_page = ListDurableObjectNamespaces {
            account_id: "test-account",
            per_page: Some(50),
            page: None,
        };
        assert_eq!(request_per_page.query(), Some("per_page=50".to_string()));

        // Test with page only
        let request_page = ListDurableObjectNamespaces {
            account_id: "test-account",
            per_page: None,
            page: Some(2),
        };
        assert_eq!(request_page.query(), Some("page=2".to_string()));

        // Test with both parameters
        let request_both = ListDurableObjectNamespaces {
            account_id: "test-account",
            per_page: Some(100),
            page: Some(3),
        };
        assert_eq!(
            request_both.query(),
            Some("per_page=100&page=3".to_string())
        );
    }

    #[test]
    fn test_list_do_objects_endpoint_path() {
        let list_request = ListDurableObjectsInNamespace {
            account_id: "test-account-456",
            namespace_id: "test-namespace-789",
            cursor: None,
            limit: None,
        };

        assert_eq!(
            list_request.path(),
            "accounts/test-account-456/workers/durable_objects/namespaces/test-namespace-789/objects"
        );
    }

    #[test]
    fn test_list_do_objects_query_params() {
        // Test with no parameters
        let request_no_params = ListDurableObjectsInNamespace {
            account_id: "test-account",
            namespace_id: "test-namespace",
            cursor: None,
            limit: None,
        };
        assert_eq!(request_no_params.query(), None);

        // Test with cursor only
        let request_cursor = ListDurableObjectsInNamespace {
            account_id: "test-account",
            namespace_id: "test-namespace",
            cursor: Some("cursor-token"),
            limit: None,
        };
        assert_eq!(
            request_cursor.query(),
            Some("cursor=cursor-token".to_string())
        );

        // Test with limit only
        let request_limit = ListDurableObjectsInNamespace {
            account_id: "test-account",
            namespace_id: "test-namespace",
            cursor: None,
            limit: Some(5000),
        };
        assert_eq!(request_limit.query(), Some("limit=5000".to_string()));

        // Test with both parameters
        let request_both = ListDurableObjectsInNamespace {
            account_id: "test-account",
            namespace_id: "test-namespace",
            cursor: Some("my-cursor"),
            limit: Some(2000),
        };
        assert_eq!(
            request_both.query(),
            Some("cursor=my-cursor&limit=2000".to_string())
        );
    }

    #[test]
    fn test_result_info_optional_fields() {
        // Test with both fields present
        let result_info_full = DurableObjectsResultInfo {
            cursor: Some("test-cursor".to_string()),
            count: Some(42),
        };

        let json_full = serde_json::to_string(&result_info_full).unwrap();
        let deserialized_full: DurableObjectsResultInfo = serde_json::from_str(&json_full).unwrap();
        assert_eq!(result_info_full, deserialized_full);

        // Test with only cursor
        let result_info_cursor = DurableObjectsResultInfo {
            cursor: Some("cursor-only".to_string()),
            count: None,
        };

        let json_cursor = serde_json::to_string(&result_info_cursor).unwrap();
        let deserialized_cursor: DurableObjectsResultInfo =
            serde_json::from_str(&json_cursor).unwrap();
        assert_eq!(result_info_cursor, deserialized_cursor);

        // Test with no fields
        let result_info_empty = DurableObjectsResultInfo {
            cursor: None,
            count: None,
        };

        let json_empty = serde_json::to_string(&result_info_empty).unwrap();
        let deserialized_empty: DurableObjectsResultInfo =
            serde_json::from_str(&json_empty).unwrap();
        assert_eq!(result_info_empty, deserialized_empty);
    }
}
