#[cfg(test)]
mod tests {
    use super::super::{
        Bucket, CreateBucket, CreateBucketParams, DeleteBucket, GetBucket, ListBuckets,
        ListBucketsResult, R2Jurisdiction, R2StorageClass, UpdateBucket, UpdateBucketParams,
    };
    use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

    #[test]
    fn test_r2_jurisdiction_serialization() {
        // Test jurisdiction enum serialization
        assert_eq!(
            serde_json::to_string(&R2Jurisdiction::Default).unwrap(),
            "\"default\""
        );
        assert_eq!(
            serde_json::to_string(&R2Jurisdiction::Eu).unwrap(),
            "\"eu\""
        );
        assert_eq!(
            serde_json::to_string(&R2Jurisdiction::Fedramp).unwrap(),
            "\"fedramp\""
        );

        // Test deserialization
        assert_eq!(
            serde_json::from_str::<R2Jurisdiction>("\"default\"").unwrap(),
            R2Jurisdiction::Default
        );
        assert_eq!(
            serde_json::from_str::<R2Jurisdiction>("\"eu\"").unwrap(),
            R2Jurisdiction::Eu
        );
        assert_eq!(
            serde_json::from_str::<R2Jurisdiction>("\"fedramp\"").unwrap(),
            R2Jurisdiction::Fedramp
        );
    }

    #[test]
    fn test_r2_storage_class_serialization() {
        // Test storage class enum serialization
        assert_eq!(
            serde_json::to_string(&R2StorageClass::Standard).unwrap(),
            "\"Standard\""
        );
        assert_eq!(
            serde_json::to_string(&R2StorageClass::InfrequentAccess).unwrap(),
            "\"InfrequentAccess\""
        );

        // Test deserialization
        assert_eq!(
            serde_json::from_str::<R2StorageClass>("\"Standard\"").unwrap(),
            R2StorageClass::Standard
        );
        assert_eq!(
            serde_json::from_str::<R2StorageClass>("\"InfrequentAccess\"").unwrap(),
            R2StorageClass::InfrequentAccess
        );
    }

    #[test]
    fn test_bucket_serialization() {
        use chrono::{DateTime, Utc};

        let bucket = Bucket {
            name: "test-bucket".to_string(),
            creation_date: DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            jurisdiction: Some(R2Jurisdiction::Eu),
            location: Some("eeur".to_string()),
            storage_class: Some(R2StorageClass::Standard),
        };

        let json = serde_json::to_string(&bucket).unwrap();
        let deserialized: Bucket = serde_json::from_str(&json).unwrap();

        assert_eq!(bucket, deserialized);
        assert_eq!(bucket.name, "test-bucket");
        assert_eq!(bucket.jurisdiction, Some(R2Jurisdiction::Eu));
        assert_eq!(bucket.location, Some("eeur".to_string()));
        assert_eq!(bucket.storage_class, Some(R2StorageClass::Standard));
    }

    #[test]
    fn test_bucket_optional_fields() {
        use chrono::{DateTime, Utc};

        let bucket = Bucket {
            name: "minimal-bucket".to_string(),
            creation_date: DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            jurisdiction: None,
            location: None,
            storage_class: None,
        };

        let json = serde_json::to_string(&bucket).unwrap();
        let deserialized: Bucket = serde_json::from_str(&json).unwrap();

        assert_eq!(bucket, deserialized);
        assert_eq!(bucket.jurisdiction, None);
        assert_eq!(bucket.location, None);
        assert_eq!(bucket.storage_class, None);
    }

    #[test]
    fn test_list_buckets_endpoint() {
        let list_request = ListBuckets {
            account_identifier: "test-account-123",
        };

        assert_eq!(list_request.method(), Method::GET);
        assert_eq!(list_request.path(), "accounts/test-account-123/r2/buckets");
        assert_eq!(list_request.query(), None);
    }

    #[test]
    fn test_create_bucket_params_serialization() {
        let params = CreateBucketParams {
            name: "new-test-bucket".to_string(),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("\"name\":\"new-test-bucket\""));
    }

    #[test]
    fn test_create_bucket_endpoint() {
        let create_request = CreateBucket {
            account_identifier: "test-account-456",
            params: CreateBucketParams {
                name: "my-bucket".to_string(),
            },
        };

        assert_eq!(create_request.method(), Method::POST);
        assert_eq!(
            create_request.path(),
            "accounts/test-account-456/r2/buckets"
        );

        // Test body serialization
        let body = create_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"name\":\"my-bucket\""));
        }
    }

    #[test]
    fn test_get_bucket_endpoint() {
        let get_request = GetBucket {
            account_identifier: "test-account-789",
            bucket_name: "existing-bucket",
        };

        assert_eq!(get_request.method(), Method::GET);
        assert_eq!(
            get_request.path(),
            "accounts/test-account-789/r2/buckets/existing-bucket"
        );
    }

    #[test]
    fn test_update_bucket_params_serialization() {
        let params = UpdateBucketParams {
            storage_class: Some(R2StorageClass::InfrequentAccess),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("\"storage_class\":\"InfrequentAccess\""));

        // Test with no storage class
        let empty_params = UpdateBucketParams {
            storage_class: None,
        };
        let empty_json = serde_json::to_string(&empty_params).unwrap();
        assert_eq!(empty_json, "{}");
    }

    #[test]
    fn test_update_bucket_endpoint() {
        let update_request = UpdateBucket {
            account_identifier: "test-account-999",
            bucket_name: "update-bucket",
            params: UpdateBucketParams {
                storage_class: Some(R2StorageClass::Standard),
            },
        };

        assert_eq!(update_request.method(), Method::PATCH);
        assert_eq!(
            update_request.path(),
            "accounts/test-account-999/r2/buckets/update-bucket"
        );

        // Test body serialization
        let body = update_request.body();
        assert!(body.is_some());
        if let Some(RequestBody::Json(json)) = body {
            assert!(json.contains("\"storage_class\":\"Standard\""));
        }
    }

    #[test]
    fn test_delete_bucket_endpoint() {
        let delete_request = DeleteBucket {
            account_identifier: "test-account-delete",
            bucket_name: "bucket-to-delete",
        };

        assert_eq!(delete_request.method(), Method::DELETE);
        assert_eq!(
            delete_request.path(),
            "accounts/test-account-delete/r2/buckets/bucket-to-delete"
        );
        assert_eq!(delete_request.query(), None);
    }

    #[test]
    fn test_list_buckets_result_serialization() {
        use chrono::{DateTime, Utc};

        let bucket1 = Bucket {
            name: "bucket-1".to_string(),
            creation_date: DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            jurisdiction: Some(R2Jurisdiction::Default),
            location: Some("wnam".to_string()),
            storage_class: Some(R2StorageClass::Standard),
        };

        let bucket2 = Bucket {
            name: "bucket-2".to_string(),
            creation_date: DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            jurisdiction: Some(R2Jurisdiction::Eu),
            location: Some("eeur".to_string()),
            storage_class: Some(R2StorageClass::InfrequentAccess),
        };

        let list_result = ListBucketsResult {
            buckets: vec![bucket1.clone(), bucket2.clone()],
        };

        let json = serde_json::to_string(&list_result).unwrap();
        let deserialized: ListBucketsResult = serde_json::from_str(&json).unwrap();

        assert_eq!(list_result, deserialized);
        assert_eq!(list_result.buckets.len(), 2);
        assert_eq!(list_result.buckets[0], bucket1);
        assert_eq!(list_result.buckets[1], bucket2);
    }

    #[test]
    fn test_endpoint_methods() {
        // Test that all endpoints have correct HTTP methods
        let create_request = CreateBucket {
            account_identifier: "test",
            params: CreateBucketParams {
                name: "test".to_string(),
            },
        };

        let get_request = GetBucket {
            account_identifier: "test",
            bucket_name: "test",
        };

        let update_request = UpdateBucket {
            account_identifier: "test",
            bucket_name: "test",
            params: UpdateBucketParams {
                storage_class: Some(R2StorageClass::Standard),
            },
        };

        let delete_request = DeleteBucket {
            account_identifier: "test",
            bucket_name: "test",
        };

        let list_request = ListBuckets {
            account_identifier: "test",
        };

        assert_eq!(create_request.method(), Method::POST);
        assert_eq!(get_request.method(), Method::GET);
        assert_eq!(update_request.method(), Method::PATCH);
        assert_eq!(delete_request.method(), Method::DELETE);
        assert_eq!(list_request.method(), Method::GET);
    }
}
