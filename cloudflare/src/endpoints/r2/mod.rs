pub mod r2;
mod tests;

pub use r2::{
    Bucket, CreateBucket, CreateBucketParams, DeleteBucket, GetBucket, ListBuckets,
    ListBucketsResult, R2Jurisdiction, R2StorageClass, UpdateBucket, UpdateBucketParams,
};
