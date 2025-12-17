use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};

/// R2 storage jurisdiction options
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum R2Jurisdiction {
    /// Default jurisdiction (US)
    Default,
    /// European Union jurisdiction
    Eu,
    /// FedRAMP compliance jurisdiction
    Fedramp,
}

/// R2 storage class options
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum R2StorageClass {
    /// Standard storage class
    Standard,
    /// Infrequent access storage class
    InfrequentAccess,
}

/// A Bucket is a collection of Objects stored in R2.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    /// Bucket name
    pub name: String,
    /// Creation date of the bucket
    pub creation_date: DateTime<Utc>,
    /// Storage jurisdiction
    pub jurisdiction: Option<R2Jurisdiction>,
    /// Bucket location
    pub location: Option<String>,
    /// Default storage class for new objects
    pub storage_class: Option<R2StorageClass>,
}

/// ListBucketsResult contains a list of buckets in an account.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ListBucketsResult {
    pub buckets: Vec<Bucket>,
}

type EmptyMap = HashMap<(), ()>;
impl ApiResult for EmptyMap {}
impl ApiResult for ListBucketsResult {}
impl ApiResult for Bucket {}

/// Lists all buckets within the account.
#[derive(Debug)]
pub struct ListBuckets<'a> {
    pub account_identifier: &'a str,
}

impl EndpointSpec for ListBuckets<'_> {
    type JsonResponse = ListBucketsResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("accounts/{}/r2/buckets", self.account_identifier)
    }
}

/// Request parameters for creating a bucket
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct CreateBucketParams {
    /// Name of the bucket to create
    pub name: String,
}

/// Creates a bucket with the given name.
/// A 400 is returned if the account already owns a bucket with this name.
/// A bucket must be explicitly deleted to be replaced.
#[derive(Debug)]
pub struct CreateBucket<'a> {
    pub account_identifier: &'a str,
    pub params: CreateBucketParams,
}

impl EndpointSpec for CreateBucket<'_> {
    type JsonResponse = Bucket;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("accounts/{}/r2/buckets", self.account_identifier)
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Gets properties of an existing R2 bucket.
#[derive(Debug)]
pub struct GetBucket<'a> {
    pub account_identifier: &'a str,
    pub bucket_name: &'a str,
}

impl EndpointSpec for GetBucket<'_> {
    type JsonResponse = Bucket;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/r2/buckets/{}",
            self.account_identifier, self.bucket_name
        )
    }
}

/// Request parameters for updating a bucket
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct UpdateBucketParams {
    /// Updated storage class for new objects
    pub storage_class: Option<R2StorageClass>,
}

/// Updates properties of an existing R2 bucket.
#[derive(Debug)]
pub struct UpdateBucket<'a> {
    pub account_identifier: &'a str,
    pub bucket_name: &'a str,
    pub params: UpdateBucketParams,
}

impl EndpointSpec for UpdateBucket<'_> {
    type JsonResponse = Bucket;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/r2/buckets/{}",
            self.account_identifier, self.bucket_name
        )
    }

    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.params).unwrap(),
        ))
    }
}

/// Deletes a bucket with the given name.
#[derive(Debug)]
pub struct DeleteBucket<'a> {
    pub account_identifier: &'a str,
    pub bucket_name: &'a str,
}

impl EndpointSpec for DeleteBucket<'_> {
    type JsonResponse = EmptyMap;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/r2/buckets/{}",
            self.account_identifier, self.bucket_name
        )
    }
}
